use sample_actor_codec::{AddRequest, AddResponse, GreetingsRequest, NAME};

use crate::{Actor, error::Result};
use tea_sdk::actorx::{ActorExt, WithActorHost, ActorId};

async fn init() -> Result<()> {
	Actor::default().register().await?;
	Ok(())
}

#[tokio::test]
async fn greeting_test() -> Result<()> {
  async {
		init().await?;
		ActorId::Static(NAME).call(
      GreetingsRequest("Alice".to_string()),
    )
    .await?;
    Ok(())
	}
	.with_actor_host()
	.await
}

#[tokio::test]
async fn greeting_empty_string_should_err() -> Result<()> {
  async {
    init().await?;
    let result = ActorId::Static(NAME).call(
      GreetingsRequest("".to_string()),
    )
    .await;

    assert!(result.is_err());
    Ok(())
  }
  .with_actor_host()
  .await
}

#[tokio::test]
async fn add_test() -> Result<()> {
  async {
    init().await?;
    let AddResponse(result) = ActorId::Static(NAME).call(AddRequest(1, 2)).await?;
    assert_eq!(result, 3);
    Ok(())
  }
  .with_actor_host()
  .await
}

