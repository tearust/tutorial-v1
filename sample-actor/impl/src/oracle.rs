use tea_sdk::utils::client_wasm_actor::{Result};
use tea_sdk::actors::http::{OracleHttpRequest};
use tea_sdk::actorx::ActorId;
use tea_sdk::actors::tappstore_client;

const TWITTER_OAUTH_KEY: &str = "AAAAAAAAAAAAAAAAAAAAAF4gTwEAAAAAedgLBTRn%2Bp78NXs2n12t7xhbcl8%3DRE8ZxY6KcGFaUKYa1F5oWgf6pE0rBC8Us8A3hiIdEUwx4rUF8f";

pub async fn twitter_request(twitter_id: &str, target_twitter_id: &str) -> Result<bool> {
  let url = format!(
    "https://api.twitter.com/2/tweets/{}?tweet.fields=referenced_tweets",
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

  check_retweet_id(json, target_twitter_id)
}

#[derive(Debug, Serialize, Deserialize)]
struct ReferencedTweets {
	pub r#type: String,
	pub id: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct RetweetCheckResult {
	pub id: String,
	pub text: String,
	pub referenced_tweets: Option<Vec<ReferencedTweets>>,
}
#[derive(Debug, Serialize, Deserialize)]
struct WrapRetweetCheckResult {
	pub data: Option<RetweetCheckResult>,
}

fn check_retweet_id(
	body: serde_json::Value,
  target_twitter_id: &str
) -> Result<bool> {
	let data: Option<RetweetCheckResult> = match serde_json::from_value(body["data"].clone()) {
		Ok(d) => Some(d),
		Err(_) => None,
	};
	let res_json = WrapRetweetCheckResult { data };
	if res_json.data.is_none() {
		return Ok(false);
	}
	let json = res_json.data.unwrap();

	if json.referenced_tweets.is_none() {
		return Ok(false);
	}
	let re_tweets: &Vec<ReferencedTweets> = &json.referenced_tweets.unwrap();
	if re_tweets.is_empty() {
		return Ok(false);
	}

	Ok(re_tweets.iter().any(|x| x.id.eq(target_twitter_id)))
}