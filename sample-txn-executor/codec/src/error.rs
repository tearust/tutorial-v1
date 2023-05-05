use tea_sdk::{actorx::error::ActorX, define_scope};

define_scope! {
    SampleActor: ActorX {
        HttpActionNotSupported;
        TxnErrors;
    }
}
