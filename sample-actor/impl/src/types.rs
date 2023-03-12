#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaucetRequest {
	pub uuid: String,
  pub address: String,
  pub tapp_id_b64: String,
  pub auth_b64: String,
}
