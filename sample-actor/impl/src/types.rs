#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaucetRequest {
	pub uuid: String,
  pub address: String,
  pub tapp_id_b64: String,
  pub auth_b64: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRequest {
  pub uuid: String,
  pub address: String,
  pub tapp_id_b64: String,
  pub auth_b64: String,

  pub subject: String,
  pub price: String,
  pub required_deposit: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTaskRequest {
  pub uuid: String,
  pub address: String,
  pub tapp_id_b64: String,
  pub auth_b64: String,

  pub subject: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyTaskRequest {
  pub uuid: String,
  pub address: String,
  pub tapp_id_b64: String,
  pub auth_b64: String,

  pub subject: String,
  pub failed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakeTaskRequest {
  pub uuid: String,
  pub address: String,
  pub tapp_id_b64: String,
  pub auth_b64: String,

  pub subject: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteTaskRequest {
  pub uuid: String,
  pub address: String,
  pub tapp_id_b64: String,
  pub auth_b64: String,

  pub subject: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryTaskRequest {
  pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitAppTokenRequest {
  pub token_id: String, 
  pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitAppDBRequest {
  pub token_id: String, 
  pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOpLogsRequest {
	pub uuid: String,
	pub target: Option<String>,
	pub year: Option<i32>,
	pub month: Option<u32>,
	pub day: Option<u32>,
}