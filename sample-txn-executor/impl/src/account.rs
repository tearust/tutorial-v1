use crate::{error::Result, sql::sum_task_deposit, utils::my_token_id};
use sample_txn_executor_codec::txn::Task;
use tea_sdk::{
    actor_txns::{context::TokenContext, Tsid},
    serialize,
    tapp::{Account, Balance, TokenId, PUBLIC_RESERVED_ACCOUNT},
    utils::wasm_actor::actors::{env::tappstore_id, tokenstate::api_cross_move},
    OptionExt, ResultExt,
};

pub(crate) async fn deposit_for_task(
    tsid: Tsid,
    base: Tsid,
    from: Account,
    amount: Balance,
    ctx: Vec<u8>,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let tappstore_ctx = tappstore_ctx(tsid, base, Some(my_token_id().await?)).await?;
    api_cross_move(from, PUBLIC_RESERVED_ACCOUNT, amount, tappstore_ctx, ctx)
        .await
        .err_into()
}

pub(crate) async fn rollback_deposit(
    tsid: Tsid,
    base: Tsid,
    task: &Task,
    ctx: Vec<u8>,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let tappstore_ctx = tappstore_ctx(tsid, base, None).await?;
    api_cross_move(
        PUBLIC_RESERVED_ACCOUNT,
        task.creator,
        task.price,
        ctx,
        tappstore_ctx,
    )
    .await
    .err_into()
}

pub(crate) async fn reward_owner(
    tsid: Tsid,
    base: Tsid,
    task: &Task,
    ctx: Vec<u8>,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let deposit_sum = sum_task_deposit(&task.subject).await?;
    let tappstore_ctx = tappstore_ctx(tsid, base, None).await?;
    let worker = task.worker.ok_or_err("worker")?;
    api_cross_move(
        PUBLIC_RESERVED_ACCOUNT,
        worker,
        deposit_sum,
        ctx,
        tappstore_ctx,
    )
    .await
    .err_into()
}

pub(crate) async fn tappstore_ctx(
    tsid: Tsid,
    base: Tsid,
    allowance_tid: Option<TokenId>,
) -> Result<Vec<u8>> {
    if let Some(token_id) = allowance_tid {
        serialize(&TokenContext::new_cross_move(
            tsid,
            base,
            tappstore_id().await?,
            token_id,
        ))
        .err_into()
    } else {
        serialize(&TokenContext::new_slim(tsid, base, tappstore_id().await?)).err_into()
    }
}
