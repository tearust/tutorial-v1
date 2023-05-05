use crate::error::{Result, TxnErrors};
use base64::{engine::general_purpose, Engine};
use tea_sdk::{
    actors::tokenstate::{QueryUserLoginSessionKeyRequest, QueryUserLoginSessionKeyResponse},
    actorx::ActorId,
    utils::wasm_actor::actors::env::get_current_wasm_actor_token_id,
    deserialize,
    tapp::{Account, AuthKey, TokenId},
    ResultExt,
};

pub async fn my_token_id() -> Result<TokenId> {
    let token_str = get_current_wasm_actor_token_id().await?;
    if token_str.is_some() {
        return Ok(TokenId::from_hex(token_str.unwrap())?);
    }
    Ok(TokenId::default())
}

pub async fn check_account(auth_b64: &str, account: Account) -> Result<()> {
    let QueryUserLoginSessionKeyResponse(expect) = ActorId::Static(tea_sdk::actors::tokenstate::NAME).call(
        QueryUserLoginSessionKeyRequest {
            token_id: my_token_id().await?,
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

