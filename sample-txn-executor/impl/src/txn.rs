use crate::{
    account,
    error::{Result, TxnErrors},
    sql::{
        complete_task, create_task, delete_task, has_task_executed, sql_init, take_task,
        task_by_subject, verify_task,
    },
    utils::{check_account, decode_auth_key, my_token_id},
};
use log::info;
use prost::Message;
use sample_txn_executor_codec::txn::{Status, Txns};
use tea_sdk::{
    actor_txns::{context::TokenContext, Tsid},
    actors::tokenstate::{SqlBeginTransactionRequest, NAME},
    actorx::{ActorId},
    serialize,
    tapp::RECEIPTING_AUTH_KEY,
    utils::wasm_actor::actors::statemachine::{query_state_tsid, CommitContext, CommitContextList},
    vmh::message::{encode_protobuf, structs_proto::tokenstate},
    OptionExt,
};

pub(crate) async fn txn_exec(tsid: Tsid, txn: &Txns) -> Result<()> {
    info!("begin of process transaction for sample => {txn}");

    let base: Tsid = query_state_tsid().await?;
    let ctx = serialize(&TokenContext::new_slim(tsid, base, my_token_id().await?))?;
    let commit_list = match txn {
        Txns::Init {} => {
            sql_init(tsid).await?;
            CommitContextList {
                ctx_list: vec![CommitContext::ctx_receipting(ctx, txn.to_string())],
                ..Default::default()
            }
        }
        Txns::CreateTask { task, auth_b64 } => {
            check_account(auth_b64, task.creator).await?;
            let (tappstore_ctx, ctx) =
                account::deposit_for_task(tsid, base, task.creator, task.price, ctx).await?;
            let glue_ctx = new_gluedb_context().await?;
            create_task(tsid, task).await?;
            CommitContextList {
                ctx_list: vec![
                    CommitContext::new(
                        ctx,
                        glue_ctx,
                        None,
                        None,
                        decode_auth_key(auth_b64)?,
                        txn.to_string(),
                    ),
                    CommitContext::ctx_receipting(tappstore_ctx, txn.to_string()),
                ],
                ..Default::default()
            }
        }
        Txns::DeleteTask { subject, auth_b64 } => {
            let task = task_by_subject(subject).await?;
            if task.status != Status::New && task.status != Status::Done {
                return Err(TxnErrors::DeleteTaskFailed.into());
            }
            if has_task_executed(subject).await? {
                return Err(TxnErrors::CanNotDeleteExecutedTask.into());
            }
            check_account(auth_b64, task.creator).await?;
            let glue_ctx = new_gluedb_context().await?;
            delete_task(tsid, subject).await?;

            if task.status == Status::New {
                let (new_ctx, tappstore_ctx) =
                    account::rollback_deposit(tsid, base, &task, ctx).await?;

                CommitContextList {
                    ctx_list: vec![
                        CommitContext::new(
                            new_ctx,
                            glue_ctx,
                            None,
                            None,
                            RECEIPTING_AUTH_KEY,
                            txn.to_string(),
                        ),
                        CommitContext::ctx_receipting(tappstore_ctx, txn.to_string()),
                    ],
                    ..Default::default()
                }
            } else {
                CommitContextList {
                    ctx_list: vec![CommitContext::new(
                        ctx,
                        glue_ctx,
                        None,
                        None,
                        decode_auth_key(auth_b64)?,
                        txn.to_string(),
                    )],
                    ..Default::default()
                }
            }
        }
        Txns::VerifyTask {
            subject,
            failed,
            auth_b64,
        } => {
            let task = task_by_subject(subject).await?;
            if task.status != Status::WaitForVerification {
                return Err(TxnErrors::VerifyTaskFailed.into());
            }
            check_account(auth_b64, task.creator).await?;
            let glue_ctx = new_gluedb_context().await?;

            verify_task(tsid, subject, *failed).await?;
            if !failed {
                let (new_ctx, tappstore_ctx) =
                    account::reward_owner(tsid, base, &task, ctx).await?;

                CommitContextList {
                    ctx_list: vec![
                        CommitContext::new(
                            new_ctx,
                            glue_ctx,
                            None,
                            None,
                            RECEIPTING_AUTH_KEY,
                            txn.to_string(),
                        ),
                        CommitContext::ctx_receipting(tappstore_ctx, txn.to_string()),
                    ],
                    ..Default::default()
                }
            } else {
                CommitContextList {
                    ctx_list: vec![CommitContext::new(
                        ctx,
                        glue_ctx,
                        None,
                        None,
                        decode_auth_key(auth_b64)?,
                        txn.to_string(),
                    )],
                    ..Default::default()
                }
            }
        }
        Txns::TakeTask {
            subject,
            worker,
            auth_b64,
        } => {
            let task = task_by_subject(subject).await?;
            if task.status != Status::New {
                return Err(TxnErrors::TakeTaskFailed.into());
            }
            if let Some(worker) = task.worker {
                return Err(TxnErrors::TaskInprogress(task.subject, worker).into());
            }
            check_account(auth_b64, *worker).await?;
            let glue_ctx = new_gluedb_context().await?;

            let (tappstore_ctx, ctx) =
                account::deposit_for_task(tsid, base, *worker, task.required_deposit, ctx).await?;
            take_task(tsid, subject, *worker, task.required_deposit).await?;
            CommitContextList {
                ctx_list: vec![
                    CommitContext::new(
                        ctx,
                        glue_ctx,
                        None,
                        None,
                        decode_auth_key(auth_b64)?,
                        txn.to_string(),
                    ),
                    CommitContext::ctx_receipting(tappstore_ctx, txn.to_string()),
                ],
                ..Default::default()
            }
        }
        Txns::CompleteTask { subject, auth_b64 } => {
            let task = task_by_subject(subject).await?;
            if task.status != Status::InProgress {
                return Err(TxnErrors::CompleteTaskFailed.into());
            }
            check_account(auth_b64, task.worker.ok_or_err("task worker")?).await?;
            let glue_ctx = new_gluedb_context().await?;
            complete_task(tsid, subject).await?;
            CommitContextList {
                ctx_list: vec![CommitContext::new(
                    ctx,
                    glue_ctx,
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
    let buf = ActorId::Static(NAME).call(
        SqlBeginTransactionRequest(encode_protobuf(tokenstate::BeginTransactionRequest {
            token_id: serialize(&my_token_id().await?)?,
        })?),
    )
    .await?;
    let res = tokenstate::BeginTransactionResponse::decode(buf.0.as_slice())?;
    Ok(res.context)
}
