use sample_actor_codec::error::SampleActor;
use tea_sdk::define_scope;
use thiserror::Error;

define_scope! {
    Impl: SampleActor {
        HttpActionNotSupported => @SampleActor::HttpActionNotSupported;
        GreetingNameEmpty => @SampleActor::GreetingNameEmpty;
    }
}

#[derive(Debug, Error)]
#[error("Http method {0} is not supported")]
pub struct HttpActionNotSupported(pub String);

#[derive(Debug, Error)]
#[error("Greeting name is empty")]
pub struct GreetingNameEmpty;
