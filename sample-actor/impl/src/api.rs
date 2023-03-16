use std::str::FromStr;

use tea_sdk::actors::tappstore::{txns::TappstoreTxn};
use tea_sdk::utils::wasm_actor::actors::{
	env::{tappstore_id},
};
use primitive_types::H160;
use tea_sdk::tapp::{DOLLARS, Account, Balance};
use crate::types::*;
use tea_sdk::utils::client_wasm_actor::{help, check_auth, request, Result};
use sample_txn_executor_codec::{
	TaskQueryRequest, TaskQueryResponse,
	txn::{Task, Status, Txns}
};

const DAO_RESERVED_ACCOUNT: Account = H160([254u8; 20]);
const TARGET_ACTOR: &'static [u8] = b"someone.sample_txn_executor";
pub async fn txn_faucet(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: FaucetRequest = serde_json::from_slice(&payload)?;
  check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("Start faucet action...");

	let txn = TappstoreTxn::TransferTea {
    token_id: tappstore_id().await?,
    from: DAO_RESERVED_ACCOUNT,
		to: req.address.parse()?,
		amount: DOLLARS * 1000,
		auth_b64: req.auth_b64.to_string(),
	};

	request::send_tappstore_txn(
		&from_actor,
		"faucet_txn",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		txn,
		vec![],
	)
	.await?;
	help::result_ok()
}

pub async fn create_task(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: CreateTaskRequest = serde_json::from_slice(&payload)?;
  check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("Create Task action...");

	let task = Task {
		creator: req.address.parse()?,
		subject: req.subject.to_string(),
		price: Balance::from(u128::from_str(&req.price)?),
		required_deposit: Balance::from(u128::from_str(&req.required_deposit)?),
		status: Status::New,
		worker: None,
	};
	let txn = Txns::CreateTask {
    task,
		auth_b64: req.auth_b64.to_string(),
	};

	request::send_custom_txn(
		&from_actor,
		"create_task",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		tea_sdk::serialize(&txn)?,
		vec![],
		TARGET_ACTOR,
	)
	.await?;

	help::result_ok()
}

pub async fn query_task_list(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: QueryTaskRequest = serde_json::from_slice(&payload)?;
	info!("Start query_task_list...");

	let uuid: String = req.uuid.to_string();

	request::send_custom_query(
		&from_actor,
		TaskQueryRequest {
			creator: None,
			worker: None,
			status: None,
			subject: None,
		},
		TARGET_ACTOR,
		move |res| {
			Box::pin(async move {
				let r: TaskQueryResponse  = res;
				let x = serde_json::json!({
					"list": r,
				});
				info!("query_task_list => {:?}", x);
				help::cache_json_with_uuid(&uuid, x).await?;
				Ok(())
			})
		},
	)
	.await?;

	help::result_ok()
}

pub async fn delete_task(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: DeleteTaskRequest = serde_json::from_slice(&payload)?;
  check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("Delete Task action...");

	let txn = Txns::DeleteTask {
    subject: req.subject.to_string(),
		auth_b64: req.auth_b64.to_string(),
	};

	request::send_custom_txn(
		&from_actor,
		"delete_task",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		tea_sdk::serialize(&txn)?,
		vec![],
		TARGET_ACTOR,
	)
	.await?;

	help::result_ok()
}

pub async fn verify_task(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: VerifyTaskRequest = serde_json::from_slice(&payload)?;
  check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("Verify Task action...");

	let txn = Txns::VerifyTask {
    subject: req.subject.to_string(),
		failed: req.failed,
		auth_b64: req.auth_b64.to_string(),
	};

	request::send_custom_txn(
		&from_actor,
		"verify_task",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		tea_sdk::serialize(&txn)?,
		vec![],
		TARGET_ACTOR,
	)
	.await?;

	help::result_ok()
}

pub async fn take_task(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: TakeTaskRequest = serde_json::from_slice(&payload)?;
  check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("Take Task action...");

	let txn = Txns::TakeTask {
    subject: req.subject.to_string(),
		worker: req.address.parse()?,
		auth_b64: req.auth_b64.to_string(),
	};

	request::send_custom_txn(
		&from_actor,
		"take_task",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		tea_sdk::serialize(&txn)?,
		vec![],
		TARGET_ACTOR,
	)
	.await?;

	help::result_ok()
}

pub async fn complete_task(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: CompleteTaskRequest = serde_json::from_slice(&payload)?;
  check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("Complete Task action...");

	let txn = Txns::CompleteTask {
    subject: req.subject.to_string(),
		auth_b64: req.auth_b64.to_string(),
	};

	request::send_custom_txn(
		&from_actor,
		"complete_task",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		tea_sdk::serialize(&txn)?,
		vec![],
		TARGET_ACTOR,
	)
	.await?;

	help::result_ok()
}