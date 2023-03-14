use tea_sdk::{actorx::runtime::error::Runtime, define_scope};

define_scope! {
    SampleActor: Runtime {
        HttpActionNotSupported;
        TxnErrors;
    }
}
