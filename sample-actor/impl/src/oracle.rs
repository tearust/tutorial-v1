use tea_sdk::utils::client_wasm_actor::{Result};
use tea_sdk::actors::http::{OracleHttpRequest};
use tea_sdk::actorx::ActorId;
use tea_sdk::actors::tappstore_client;

const TWITTER_OAUTH_KEY: &str = "AAAAAAAAAAAAAAAAAAAAAF4gTwEAAAAAedgLBTRn%2Bp78NXs2n12t7xhbcl8%3DRE8ZxY6KcGFaUKYa1F5oWgf6pE0rBC8Us8A3hiIdEUwx4rUF8f";

pub async fn twitter_request(twitter_id: &str, target_str: &str) -> Result<bool> {
  let url = format!(
		"https://api.twitter.com/2/tweets/{}",
		twitter_id
	);

  let headers = vec![(
		"Authorization".to_string(),
		format!("Bearer {TWITTER_OAUTH_KEY}"),
	)];

  let req = OracleHttpRequest {
    method: "GET".to_string(),
    url,
    headers: Some(headers),
    payload: None
  };

  let rs = ActorId::Static(tappstore_client::NAME).call(
    req,
  ).await?;
  let json: serde_json::Value = serde_json::from_str(&rs.text)?;

  if let Some(twitter_text) = json["data"]["text"].as_str() {
    return Ok(twitter_text.contains(target_str));
  }

  Ok(false)
}