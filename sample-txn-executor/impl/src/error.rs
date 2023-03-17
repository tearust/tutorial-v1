use sample_txn_executor_codec::error::SampleActor;
use tea_sdk::{define_scope, tapp::Account};
use thiserror::Error;

define_scope! {
    Impl: SampleActor {
        HttpActionNotSupported => @SampleActor::HttpActionNotSupported;
        TxnErrors => @SampleActor::TxnErrors;
    }
}

#[derive(Debug, Error)]
#[error("Http method {0} is not supported")]
pub struct HttpActionNotSupported(pub String);

#[derive(Debug, Error)]
pub enum TxnErrors {
    #[error("Account {0:?} is not allowed to operate task")]
    InvalidAccount(Account),

    #[error("Task {0} already token by {1:?}")]
    TaskInprogress(String, Account),

    #[error("Task can only be deleted when status is new or done")]
    DeleteTaskFailed,

    #[error("Task can only be verified when status is wait for verification")]
    VerifyTaskFailed,

    #[error("Task can only be taken when status is new")]
    TakeTaskFailed,

    #[error("Task can only be finished when status is in process")]
    CompleteTaskFailed,

    #[error("This task has been executed (with failed results) that can't be deleted")]
    CanNotDeleteExecutedTask,
}
