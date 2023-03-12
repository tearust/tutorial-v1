use crate::error::Result;
use crate::api;
pub fn name_list() -> Vec<&'static str> {
	vec![
		"say-hello",
		"faucet",
	]
}

pub async fn map_handler(action: &str, arg: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let res = match action {
		"say-hello" => serde_json::to_vec("Hello world!").unwrap(),
		"faucet" => api::txn_faucet(arg, from_actor).await?,
		_ => vec![],
	};
	Ok(res)
}