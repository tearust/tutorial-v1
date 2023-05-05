#![feature(min_specialization)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use error::{HttpActionNotSupported, Result};
use log::{error, info};
use sample_txn_executor_codec::{
    NAME,
    txn::{Task, Txns},
    *,
};
use sql::query_all_tasks;
use tea_sdk::{
    actors::{adapter::HttpRequest, replica::ExecTxnCast, state_receiver::ActorTxnCheckMessage},
    actorx::{actor, ActorId, hooks::Activate, HandlerActor},
    deserialize,
    serde::handle::{Handle, Handles},
    utils::wasm_actor::{
        action::process_txn_error, actors::adapter::register_adapter_http_dispatcher,
        logging::set_logging,
    },
    Handle, ResultExt,
};

mod account;
pub mod error;
mod sql;
mod txn;
mod utils;

actor!(Actor);

#[derive(Default, Clone)]
pub struct Actor;

impl Handles for Actor {
    type List = Handle![
        Activate,
        HttpRequest,
        TaskQueryRequest,
        ExecTxnCast,
        ActorTxnCheckMessage
    ];
}

impl HandlerActor for Actor {
	fn id(&self) -> Option<ActorId> {
		Some(NAME.into())
	}

	async fn pre_handle<'a>(&'a self, req: &'a [u8]) -> Result<std::borrow::Cow<'a, [u8]>> {
		set_logging(false, false);
		Ok(std::borrow::Cow::Borrowed(req))
	}
}

impl Handle<Activate> for Actor {
    async fn handle(&self, _: Activate) -> Result<()> {
        info!("activate sample txn executor actor successfully");
        register_adapter_http_dispatcher(vec!["query-tasks".to_string()]).await?;

        info!("activate sample txn executor actor successfully.");
        Ok(())
    }
}

impl Handle<HttpRequest> for Actor {
    async fn handle(&self, HttpRequest { action, payload }: HttpRequest) -> Result<Vec<u8>> {
        match action.as_str() {
            "query-tasks" => {
                let query: TaskQueryRequest = serde_json::from_slice(&payload)?;
                let tasks = query_tasks_by_filter(query).await?;
                serde_json::to_vec(&tasks).err_into()
            }
            _ => Err(HttpActionNotSupported(action).into()),
        }
    }
}

impl Handle<TaskQueryRequest> for Actor {
    async fn handle(&self, req: TaskQueryRequest) -> Result<TaskQueryResponse> {
        Ok(TaskQueryResponse(query_tasks_by_filter(req).await?))
    }
}

impl Handle<ExecTxnCast> for Actor {
    async fn handle(&self, ExecTxnCast(tsid, txn_bytes, _args): ExecTxnCast) -> Result<()> {
        let txn: Txns = deserialize(txn_bytes)?;
        if let Err(e) = txn::txn_exec(tsid, &txn).await {
            error!("exec txn error: {}", e);
            process_txn_error(tsid, e.into()).await?;
        }
        Ok(())
    }
}

impl Handle<ActorTxnCheckMessage> for Actor {
    async fn handle(&self, req: ActorTxnCheckMessage) -> Result<()> {
        let _txn: Txns = deserialize(req.txn_bytes.as_slice())?;
        // all transaction types are allowed to send from b nodes,
        //  so there is no additional check.
        Ok(())
    }
}

async fn query_tasks_by_filter(req: TaskQueryRequest) -> Result<Vec<Task>> {
    // we simply query all tasks here, considering add filters in sql for higher performance
    let tasks = query_all_tasks().await?;
    Ok(tasks
        .into_iter()
        .filter(|v| {
            req.creator
                .map_or_else(|| true, |creator| creator == v.creator)
        })
        .filter(|v| {
            req.worker
                .map_or_else(|| true, |worker| Some(worker) == v.worker)
        })
        .filter(|v| req.status.map_or_else(|| true, |status| status == v.status))
        .filter(|v| {
            req.subject
                .as_ref()
                .map_or_else(|| true, |subject| subject == &v.subject)
        })
        .collect())
}
