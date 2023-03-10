use sample_actor_codec::{AddRequest, AddResponse, GreetingsRequest};
use tea_sdk::actorx::{
    runtime::{call, ActorHost, MockedActorName, RegisterMocked},
    RegId,
};

use crate::{error::Result, Actor};

#[tea_sdk::test(init)]
async fn greeting_test() -> Result<()> {
    call(
        RegId::from(Actor::NAME).inst(0),
        GreetingsRequest("Alice".to_string()),
    )
    .await?;

    Ok(())
}

#[tea_sdk::test(init)]
async fn greeting_empty_string_should_err() -> Result<()> {
    let result: Result<_> = call(
        RegId::from(Actor::NAME).inst(0),
        GreetingsRequest("".to_string()),
    )
    .await;

    assert!(result.is_err());

    Ok(())
}

#[tea_sdk::test(init)]
async fn add_test() -> Result<()> {
    let AddResponse(result) = call(RegId::from(Actor::NAME).inst(0), AddRequest(1, 2)).await?;

    assert_eq!(result, 3);

    Ok(())
}

async fn init() -> Result<ActorHost> {
    let host = ActorHost::new();
    host.register_mocked(Actor)?;
    Ok(host)
}

impl MockedActorName for Actor {
    const NAME: &'static [u8] = b"someone.sample";
}
