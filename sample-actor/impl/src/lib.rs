#![feature(min_specialization)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use crate::error::GreetingNameEmpty;
use error::{Result};
use sample_actor_codec::{GreetingsRequest, NAME};
use tea_sdk::{
    actorx::{actor, hooks::Activate, HandlerActor, ActorId},
    serde::handle::{Handle, Handles},
    utils::wasm_actor::{
        logging::set_logging,
    },
    Handle,
};
use tea_sdk::utils::client_wasm_actor::types::{map_handler, HttpRequest};

use ::{log::info, tea_sdk::utils::wasm_actor::actors::adapter::register_adapter_http_dispatcher};
use tea_sdk::utils::client_wasm_actor::types::map_fn_list;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

pub mod error;
mod types;
mod dfn;
mod api;

actor!(Actor);

#[derive(Default, Clone)]
pub struct Actor;

impl Handles for Actor {
    type List = Handle![
        Activate,
        HttpRequest,
        GreetingsRequest
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
        let list = [&map_fn_list()[..], &crate::dfn::name_list()[..]].concat();
        register_adapter_http_dispatcher(list.iter().map(|v| v.to_string()).collect()).await?;
        info!("activate sample actor successfully");
        Ok(())
    }

}

impl Handle<HttpRequest> for Actor {
	async fn handle(&self, req: HttpRequest) -> Result<Vec<u8>> {
		let from_actor = String::from_utf8(NAME.to_vec())?;
		let base_res = map_handler(&req.action, req.clone().payload, from_actor.clone()).await?;
		let cur_res = crate::dfn::map_handler(&req.action, req.payload, from_actor).await?;
		if cur_res.is_empty() && !base_res.is_empty() {
			return Ok(base_res);
		}
		Ok(cur_res)
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

