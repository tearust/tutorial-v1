use crate::error::Result;
pub fn name_list() -> Vec<&'static str> {
	vec![
		"say-hello",
	]
}

pub async fn map_handler(action: &str, _arg: Vec<u8>, _from_actor: String) -> Result<Vec<u8>> {
	let res = match action {
		"say-hello" => serde_json::to_vec("Hello world!").unwrap(),
		_ => vec![],
	};
	Ok(res)
}