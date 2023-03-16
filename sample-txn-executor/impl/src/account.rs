use crate::{error::Result, sql::sum_task_deposit};
use sample_txn_executor_codec::txn::Task;
use tea_sdk::{
    actor_txns::{context::TokenContext, Tsid},
    serialize,
    tapp::{Account, Balance},
    utils::wasm_actor::{
        actors::{
            env::tappstore_id,
            tokenstate::{cross_move, mov},
        },
        prelude::*,
    },
    OptionExt, ResultExt,
};

pub const TASK_RESERVED_ACCOUNT: Account = H160([10_u8; 20]);

pub(crate) async fn deposit_for_task(
    tsid: Tsid,
    base: Tsid,
    from: Account,
    amount: Balance,
    ctx: Vec<u8>,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let tappstore_ctx = tappstore_ctx(tsid, base).await?;
    cross_move(from, TASK_RESERVED_ACCOUNT, amount, tappstore_ctx, ctx)
        .await
        .err_into()
}

pub(crate) async fn rollback_deposit(
    tsid: Tsid,
    base: Tsid,
    task: &Task,
    ctx: Vec<u8>,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let tappstore_ctx = tappstore_ctx(tsid, base).await?;
    cross_move(
        TASK_RESERVED_ACCOUNT,
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
    mut ctx: Vec<u8>,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let deposit_sum = sum_task_deposit(&task.subject).await?;
    let tappstore_ctx = tappstore_ctx(tsid, base).await?;
    let worker = task.worker.ok_or_err("worker")?;
    ctx = mov(TASK_RESERVED_ACCOUNT, worker, deposit_sum, ctx).await?;
    cross_move(worker, worker, deposit_sum, ctx, tappstore_ctx)
        .await
        .err_into()
}

pub(crate) async fn tappstore_ctx(tsid: Tsid, base: Tsid) -> Result<Vec<u8>> {
    serialize(&TokenContext::new_slim(tsid, base, tappstore_id().await?)).err_into()
}
