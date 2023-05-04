use crate::error::{Result, TxnErrors};
use base64::{engine::general_purpose, Engine};
use tea_sdk::{
    actors::tokenstate::{QueryUserLoginSessionKeyRequest, QueryUserLoginSessionKeyResponse},
    actorx::ActorId,
    deserialize,
    tapp::{Account, AuthKey, TokenId},
    ResultExt,
};

pub fn my_token_id() -> TokenId {
    // token id should equal to `token_id` field defined in manifest.yaml
    TokenId::default()
}

pub async fn check_account(auth_b64: &str, account: Account) -> Result<()> {
    let QueryUserLoginSessionKeyResponse(expect) = ActorId::Static(tea_sdk::actors::tokenstate::NAME).call(
        QueryUserLoginSessionKeyRequest {
            token_id: my_token_id(),
            account,
        },
    )
    .await?;
    let actual = decode_auth_key(auth_b64)?;
    if expect != Some(actual) {
        Err(TxnErrors::InvalidAccount(account).into())
    } else {
        Ok(())
    }
}

pub fn decode_auth_key(auth_b64: &str) -> Result<AuthKey> {
    deserialize(general_purpose::STANDARD.decode(auth_b64)?).err_into()
}

