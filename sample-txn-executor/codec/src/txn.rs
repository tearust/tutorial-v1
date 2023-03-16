use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};
use tea_sdk::tapp::{Account, Balance};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub creator: Account,
    pub subject: String,
    pub price: Balance,
    pub required_deposit: Balance,
    pub status: Status,
    pub worker: Option<Account>,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, AsRefStr, EnumString, Display,
)]
pub enum Status {
    New,
    InProgress,
    WaitForVerification,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsRefStr, Display)]
pub enum Txns {
    Init {},
    CreateTask {
        task: Task,
        auth_b64: String,
    },
    DeleteTask {
        subject: String,
        auth_b64: String,
    },
    VerifyTask {
        subject: String,
        failed: bool,
        auth_b64: String,
    },
    TakeTask {
        subject: String,
        worker: Account,
        auth_b64: String,
    },
    CompleteTask {
        subject: String,
        auth_b64: String,
    },
}
