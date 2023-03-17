use crate::{
    error::{Result, TxnErrors},
    sql::{
        complete_task, create_task, delete_task, sql_init, take_task, task_by_subject, verify_task,
    },
    utils::{check_account, decode_auth_key, my_token_id},
};
use log::info;
use prost::Message;
use sample_txn_executor_codec::txn::Txns;
use tea_sdk::{
    actor_txns::{context::TokenContext, Tsid, TxnSerial},
    actors::{tokenstate::{SqlBeginTransactionRequest, NAME}, tappstore::txns::TappstoreTxn, tappstore::NAME as TAPPSTORE_NAME},
    actorx::{runtime::call, RegId},
    serialize,
    utils::wasm_actor::actors::statemachine::{query_state_tsid, CommitContext, CommitContextList},
    vmh::message::{encode_protobuf, structs_proto::tokenstate},
    OptionExt, tapp::GOD_MODE_AUTH_KEY,
};

pub(crate) async fn txn_exec(tsid: Tsid, txn: &Txns) -> Result<()> {
    info!("begin of process transaction for sample => {txn}");

    let base: Tsid = query_state_tsid().await?;
    let ctx = serialize(&TokenContext::new_slim(tsid, base, my_token_id()))?;
    let commit_ctx = match txn {
        Txns::Init {} => {
            sql_init(tsid).await?;
            // init_app_token().await?;
            CommitContext::new(
                ctx,
                None,
                None,
                None,
                GOD_MODE_AUTH_KEY,
                txn.to_string(),
            )
        }
        Txns::CreateTask { task, auth_b64 } => {
            check_account(auth_b64, task.creator).await?;
            let glue_ctx = new_gluedb_context().await?;
            create_task(tsid, task).await?;
            CommitContext::new(
                ctx,
                glue_ctx,
                None,
                None,
                decode_auth_key(auth_b64)?,
                txn.to_string(),
            )
        }
        Txns::DeleteTask { subject, auth_b64 } => {
            let task = task_by_subject(subject).await?;
            check_account(auth_b64, task.creator).await?;
            let glue_ctx = new_gluedb_context().await?;
            delete_task(tsid, subject).await?;
            CommitContext::new(
                ctx,
                glue_ctx,
                None,
                None,
                decode_auth_key(auth_b64)?,
                txn.to_string(),
            )
        }
        Txns::VerifyTask {
            subject,
            failed,
            auth_b64,
        } => {
            let task = task_by_subject(subject).await?;
            check_account(auth_b64, task.creator).await?;
            let glue_ctx = new_gluedb_context().await?;
            verify_task(tsid, subject, *failed).await?;
            CommitContext::new(
                ctx,
                glue_ctx,
                None,
                None,
                decode_auth_key(auth_b64)?,
                txn.to_string(),
            )
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
            check_account(auth_b64, *worker).await?;
            let glue_ctx = new_gluedb_context().await?;
            take_task(tsid, subject, *worker).await?;
            CommitContext::new(
                ctx,
                glue_ctx,
                None,
                None,
                decode_auth_key(auth_b64)?,
                txn.to_string(),
            )
        }
        Txns::CompleteTask { subject, auth_b64 } => {
            let task = task_by_subject(subject).await?;
            check_account(auth_b64, task.worker.ok_or_err("task worker")?).await?;
            let glue_ctx = new_gluedb_context().await?;
            complete_task(tsid, subject).await?;
            CommitContext::new(
                ctx,
                glue_ctx,
                None,
                None,
                decode_auth_key(auth_b64)?,
                txn.to_string(),
            )
        }
    };
    CommitContextList {
        ctx_list: vec![commit_ctx],
        ..Default::default()
    }
    .commit(base, tsid)
    .await?;
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

// pub async fn init_app_db() -> Result<()> {
//     let txn_bytes = tea_sdk::serialize(&Txns::Init{})?;
//     tea_sdk::utils::wasm_actor::actors::replica::send_transaction_locally_ex(
//         &TxnSerial::new(
//             "someone.sample_txn_executor".as_bytes().to_vec(),
//             txn_bytes,
//             tea_sdk::utils::wasm_actor::actors::enclave::random_u64().await?,
//             u64::MAX,
//         ),
//         None,
//         true,
//     )
//     .await?;
//     Ok(())
// }

pub async fn init_app_token() -> Result<()> {
    let token_id = my_token_id();
    let txn = TappstoreTxn::GenAesKey { token_id };
    let txn_bytes = tea_sdk::serialize(&txn)?;
    tea_sdk::utils::wasm_actor::actors::replica::send_transaction_locally_ex(
        &TxnSerial::new(
            TAPPSTORE_NAME.to_vec(),
            txn_bytes,
            tea_sdk::utils::wasm_actor::actors::enclave::random_u64().await?,
            u64::MAX,
        ),
        None,
        true,
    )
    .await?;
    Ok(())
}