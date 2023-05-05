#![feature(min_specialization)]

use serde::{Deserialize, Serialize};
use tea_sdk::serde::TypeId;

pub mod error;

pub const NAME: &[u8] = b"com.developer.sample-actor";

#[derive(Debug, Clone, Serialize, Deserialize, TypeId)]
#[response(())]
pub struct GreetingsRequest(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, TypeId)]
pub struct AddRequest(pub i32, pub i32);

#[derive(Debug, Clone, Serialize, Deserialize, TypeId)]
pub struct AddResponse(pub i32);
