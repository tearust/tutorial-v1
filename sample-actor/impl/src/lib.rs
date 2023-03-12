#![feature(min_specialization)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use crate::error::GreetingNameEmpty;
use error::{Result};
use sample_actor_codec::{GreetingsRequest};
use tea_sdk::{
    actorx::runtime::{actor, println, Activate, PreInvoke},
    serde::handle::{Handle, Handles},
    utils::wasm_actor::{
        action::callback_reply,
        logging::set_logging,
    },
    Handle, ResultExt,
};
use tea_sdk::utils::client_wasm_actor::types::{map_handler, HttpRequest};
use tea_sdk::actors::libp2p::Libp2pReply;

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

impl Handles<()> for Actor {
    type List = Handle![
        Activate,
        PreInvoke,
        HttpRequest,
        GreetingsRequest,
        Libp2pReply
    ];
}

impl Handle<(), Activate> for Actor {
    async fn handle(self, _: Activate, _: ()) -> Result<()> {
        let list = [&map_fn_list()[..], &crate::dfn::name_list()[..]].concat();
        register_adapter_http_dispatcher(list.iter().map(|v| v.to_string()).collect()).await?;
        info!("activate sample actor successfully");
        Ok(())
    }

}

impl Handle<(), PreInvoke> for Actor {
    async fn handle(self, _: PreInvoke, _: ()) -> Result<()> {
        set_logging(false, false);
        Ok(())
    }
}


impl Handle<(), HttpRequest> for Actor {
	async fn handle(self, req: HttpRequest, _: ()) -> Result<Vec<u8>> {
		let from_actor = "sample_actor".to_string();
		let base_res = map_handler(&req.action, req.clone().payload, from_actor.clone()).await?;
		let cur_res = crate::dfn::map_handler(&req.action, req.payload, from_actor).await?;
		if cur_res.is_empty() && !base_res.is_empty() {
			return Ok(base_res);
		}
		Ok(cur_res)
	}
}

impl Handle<(), GreetingsRequest> for Actor {
    async fn handle(self, GreetingsRequest(name): GreetingsRequest, _: ()) -> Result<()> {
        if name.is_empty() {
            return Err(GreetingNameEmpty.into());
        }

        println!("Hello, {name}!");
        Ok(())
    }
}


impl Handle<(), Libp2pReply> for Actor {
	async fn handle(self, Libp2pReply(seq_number, payload): Libp2pReply, _: ()) -> Result<()> {
		callback_reply(seq_number, payload).await.err_into()
	}
}
