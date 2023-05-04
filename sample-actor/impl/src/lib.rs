#![feature(min_specialization)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use crate::error::GreetingNameEmpty;
use error::{HttpActionNotSupported, Result};
use sample_actor_codec::{AddRequest, AddResponse, GreetingsRequest, NAME};
use tea_sdk::{
    actors::adapter::HttpRequest,
    actorx::hooks::{Activate},
    actorx::{actor, ActorId, HandlerActor},
    serde::handle::{Handle, Handles},
    utils::wasm_actor::logging::set_logging,
    Handle, ResultExt,
};

#[cfg(not(test))]
use ::{log::info, tea_sdk::utils::wasm_actor::actors::adapter::register_adapter_http_dispatcher};

pub mod error;
#[cfg(test)]
mod tests;

actor!(Actor);

#[derive(Default, Clone)]
pub struct Actor;

impl Handles for Actor {
    type List = Handle![
        Activate,
        HttpRequest,
        GreetingsRequest,
        AddRequest
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
        #[cfg(not(test))]
        {
            register_adapter_http_dispatcher(vec!["say-hello".to_string()]).await?;
            info!("activate sample actor successfully");
        }
        Ok(())
    }
}


impl Handle<HttpRequest> for Actor {
    async fn handle(&self, HttpRequest { action, .. }: HttpRequest) -> Result<Vec<u8>> {
        log::info!("@@ aa => {:?}", action);
        match action.as_str() {
            "say-hello" => serde_json::to_vec("Hello world!").err_into(),
            _ => Err(HttpActionNotSupported(action).into()),
        }
    }
}

impl Handle<GreetingsRequest> for Actor {
    async fn handle(&self, GreetingsRequest(name): GreetingsRequest) -> Result<()> {
        if name.is_empty() {
            return Err(GreetingNameEmpty.into());
        }

        println!("Hello, {name}!");
        Ok(())
    }
}

impl Handle<AddRequest> for Actor {
    async fn handle(&self, AddRequest(lhs, rhs): AddRequest) -> Result<AddResponse> {
        Ok(AddResponse(lhs + rhs))
    }
}
