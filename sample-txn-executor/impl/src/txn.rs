use crate::{
    account,
    error::{Result, TxnErrors},
    sql::{
        complete_task, create_task, delete_task, sql_init, take_task, task_by_subject, verify_task,
    },
    utils::{check_account, decode_auth_key, my_token_id},
};
use log::info;
use prost::Message;
use sample_txn_executor_codec::txn::{Status, Txns};
use tea_sdk::{
    actor_txns::{context::TokenContext, Tsid, TxnSerial},
    actors::tokenstate::{SqlBeginTransactionRequest, NAME},
    actorx::{runtime::call, RegId},
    serialize,
    tapp::GOD_MODE_AUTH_KEY,
    utils::wasm_actor::actors::statemachine::{query_state_tsid, CommitContext, CommitContextList},
    vmh::message::{encode_protobuf, structs_proto::tokenstate},
    OptionExt,
};

pub(crate) async fn txn_exec(tsid: Tsid, txn: &Txns) -> Result<()> {
    info!("begin of process transaction for sample => {txn}");

    let base: Tsid = query_state_tsid().await?;
    let mut ctx = serialize(&TokenContext::new_slim(tsid, base, my_token_id()))?;
    let commit_list = match txn {
        Txns::Init { auth_b64 } => {
            // TODO: check account is tapp owner later
            sql_init(tsid).await?;
            CommitContextList {
                ctx_list: vec![CommitContext::new(
                    ctx,
                    None,
                    None,
                    None,
                    decode_auth_key(auth_b64)?,
                    txn.to_string(),
                )],
                ..Default::default()
            }
        }
        Txns::CreateTask { task, auth_b64 } => {
            // check_account(auth_b64, task.creator).await?;
            let (tappstore_ctx, ctx) =
                account::deposit_for_task(tsid, base, task.creator, task.price, ctx).await?;
            create_task(tsid, task).await?;
            CommitContextList {
                ctx_list: vec![
                    CommitContext::new(
                        ctx,
                        new_gluedb_context().await?,
                        None,
                        None,
                        decode_auth_key(auth_b64)?,
                        txn.to_string(),
                    ),
                    CommitContext::new(
                        tappstore_ctx,
                        None,
                        None,
                        None,
                        GOD_MODE_AUTH_KEY,
                        txn.to_string(),
                    ),
                ],
                ..Default::default()
            }
        }
        Txns::DeleteTask { subject, auth_b64 } => {
            let task = task_by_subject(subject).await?;
            // check_account(auth_b64, task.creator).await?;
            delete_task(tsid, subject).await?;
            let mut tappstore_ctx = None;
            if task.status == Status::New {
                let (new_ctx, tappstore) =
                    account::rollback_deposit(tsid, base, &task, ctx).await?;
                ctx = new_ctx;
                tappstore_ctx = Some(tappstore);
            }

            let commit_ctx = CommitContext::new(
                ctx,
                glue_ctx,
                None,
                None,
                decode_auth_key(auth_b64)?,
                txn.to_string(),
            );
            CommitContextList {
                ctx_list: match tappstore_ctx {
                    Some(tappstore_ctx) => vec![
                        commit_ctx,
                        CommitContext::new(
                            tappstore_ctx,
                            None,
                            None,
                            None,
                            GOD_MODE_AUTH_KEY,
                            txn.to_string(),
                        ),
                    ],
                    None => vec![commit_ctx],
                },
                ..Default::default()
            }
        }
        Txns::VerifyTask {
            subject,
            failed,
            auth_b64,
        } => {
            let task = task_by_subject(subject).await?;
            // check_account(auth_b64, task.creator).await?;
            let mut tappstore_ctx = None;
            if !failed {
                let (new_ctx, tappstore) = account::reward_owner(tsid, base, &task, ctx).await?;
                ctx = new_ctx;
                tappstore_ctx = Some(tappstore);
            }
            verify_task(tsid, subject, *failed).await?;

            let commit_ctx = CommitContext::new(
                ctx,
                glue_ctx,
                None,
                None,
                decode_auth_key(auth_b64)?,
                txn.to_string(),
            );
            CommitContextList {
                ctx_list: match tappstore_ctx {
                    Some(tappstore_ctx) => vec![
                        commit_ctx,
                        CommitContext::new(
                            tappstore_ctx,
                            None,
                            None,
                            None,
                            GOD_MODE_AUTH_KEY,
                            txn.to_string(),
                        ),
                    ],
                    None => vec![commit_ctx],
                },
                ..Default::default()
            }
        }
        Txns::TakeTask {
            subject,
            worker,
            auth_b64,
        } => {
            let task = task_by_subject(subject).await?;
            if let Some(worker) = task.worker {
                return Err(TxnErrors::TaskInprogress(task.subject, worker).into());
            }
            // check_account(auth_b64, *worker).await?;
            let (tappstore_ctx, ctx) =
                account::deposit_for_task(tsid, base, *worker, task.required_deposit, ctx).await?;
            take_task(tsid, subject, *worker, task.price).await?;
            CommitContextList {
                ctx_list: vec![
                    CommitContext::new(
                        ctx,
                        new_gluedb_context().await?,
                        None,
                        None,
                        decode_auth_key(auth_b64)?,
                        txn.to_string(),
                    ),
                    CommitContext::new(
                        tappstore_ctx,
                        None,
                        None,
                        None,
                        GOD_MODE_AUTH_KEY, // TODO:
                        txn.to_string(),
                    ),
                ],
                ..Default::default()
            }
        }
        Txns::CompleteTask { subject, auth_b64 } => {
            let task = task_by_subject(subject).await?;
            // check_account(auth_b64, task.worker.ok_or_err("task worker")?).await?;
            complete_task(tsid, subject).await?;
            CommitContextList {
                ctx_list: vec![CommitContext::new(
                    ctx,
                    new_gluedb_context().await?,
                    None,
                    None,
                    decode_auth_key(auth_b64)?,
                    txn.to_string(),
                )],
                ..Default::default()
            }
        }
    };

    commit_list.commit(base, tsid).await?;
    info!("transaction {txn} committed successfully");
    Ok(())
}

async fn new_gluedb_context() -> Result<Option<tokenstate::GluedbTransactionContext>> {
    let buf = call(
        RegId::Static(NAME).inst(0),
        SqlBeginTransactionRequest(encode_protobuf(tokenstate::BeginTransactionRequest {
            token_id: serialize(&my_token_id())?,
        })?),
    )
    .await?;
    let res = tokenstate::BeginTransactionResponse::decode(buf.0.as_slice())?;
    Ok(res.context)
}

pub async fn send_local_tx(txn: Txns) -> Result<()> {
    let txn_bytes: Vec<u8> = serialize(&txn)?;
    let txn_name = txn.to_string();

    let tsid = tea_sdk::utils::wasm_actor::actors::replica::send_transaction_locally_ex(
        &TxnSerial::new(
            "someone.sample_txn_executor".as_bytes().to_vec(),
            txn_bytes,
            tea_sdk::utils::wasm_actor::actors::enclave::random_u64().await?,
            u64::MAX,
        ),
        None,
        true,
    )
    .await?;
    info!(
        "send sample-txn-exectuor {} transaction result: {:?}",
        txn_name, tsid
    );
    Ok(())
}
