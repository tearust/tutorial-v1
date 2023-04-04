use crate::{error::Result, utils::my_token_id};
use log::info;
use sample_txn_executor_codec::txn::*;
use tea_sdk::{
    actor_txns::Tsid,
    actors::tokenstate::{ExecGlueCmdRequest, InitGlueSqlRequest, NAME},
    actorx::{runtime::call, RegId},
    serialize,
    tapp::{Account, Balance},
    utils::wasm_actor::{
        actors::tokenstate::{
            query_first_row, query_select_rows, sql_query_first, sql_value_to_option_string,
            sql_value_to_string,
        },
        prelude::Row,
    },
    vmh::message::{encode_protobuf, structs_proto::tokenstate},
    OptionExt,
};

pub(crate) async fn query_all_tasks() -> Result<Vec<Task>> {
    let payload = sql_query_first(my_token_id().await?, "SELECT * FROM Tasks;".into()).await?;
    let rows = query_select_rows(&payload)?;
    rows.iter().map(|v| parse_task(v)).collect()
}

pub(crate) async fn task_by_subject(subject: &str) -> Result<Task> {
    let payload = sql_query_first(
        my_token_id().await?,
        format!("SELECT * FROM Tasks where subject = '{subject}';"),
    )
    .await?;
    let r = query_first_row(&payload)?;
    parse_task(r)
}

pub(crate) async fn sum_task_deposit(subject: &str) -> Result<Balance> {
    let payload = sql_query_first(
        my_token_id().await?,
        format!("SELECT price FROM TaskExecution where subject='{subject}';"),
    )
    .await?;
    let rows = query_select_rows(&payload)?;
    let mut sum = Balance::zero();
    for row in rows {
        let price = Balance::from_str_radix(
            &sql_value_to_string(row.get_value_by_index(0).ok_or_err("price")?)?,
            10,
        )?;
        sum = sum.checked_add(price).ok_or_err("add overflow")?;
    }
    Ok(sum)
}

pub(crate) async fn has_task_executed(subject: &str) -> Result<bool> {
    let payload = sql_query_first(
        my_token_id().await?,
        format!("SELECT price FROM TaskExecution where subject='{subject}';"),
    )
    .await?;
    let rows = query_select_rows(&payload)?;
    Ok(rows.len() > 1)
}

pub(crate) async fn create_task(tsid: Tsid, task: &Task) -> Result<()> {
    exec_sql(
        tsid,
        format!(
            r#"
            INSERT INTO Tasks VALUES (
                '{subject}','{creator:?}',NULL,'{status}','{price}','{required_deposit}'
            );
            INSERT INTO TaskExecution VALUES (
                '{subject}', '{creator:?}', '{price}'
            );
            "#,
            subject = task.subject,
            creator = task.creator,
            status = Status::New,
            price = task.price,
            required_deposit = task.required_deposit
        ),
    )
    .await
}

pub(crate) async fn delete_task(tsid: Tsid, subject: &str) -> Result<()> {
    exec_sql(
        tsid,
        format!(
            r#"
            DELETE FROM Tasks WHERE subject = '{subject}';
            DELETE FROM TaskExecution WHERE subject = '{subject}';
        "#
        ),
    )
    .await
}

pub(crate) async fn verify_task(tsid: Tsid, subject: &str, failed: bool) -> Result<()> {
    let sql = if failed {
        format!(
            "UPDATE Tasks SET status = '{}', worker = NULL WHERE subject = '{subject}';",
            Status::New
        )
    } else {
        format!(
            "UPDATE Tasks SET status = '{}' WHERE subject = '{subject}';",
            Status::Done
        )
    };
    exec_sql(tsid, sql).await
}

pub(crate) async fn take_task(
    tsid: Tsid,
    subject: &str,
    worker: Account,
    required_deposit: Balance,
) -> Result<()> {
    exec_sql(
        tsid,
        format!(
            r#"
            UPDATE Tasks SET 
                status = '{}',worker = '{worker:?}' 
                WHERE subject = '{subject}';
            INSERT INTO TaskExecution VALUES (
                '{subject}', '{worker:?}', '{required_deposit}'
            );
               "#,
            Status::InProgress
        ),
    )
    .await
}

pub(crate) async fn complete_task(tsid: Tsid, subject: &str) -> Result<()> {
    exec_sql(
        tsid,
        format!(
            "UPDATE Tasks SET status = '{}' WHERE subject = '{subject}';",
            Status::WaitForVerification
        ),
    )
    .await
}

pub(crate) async fn sql_init(tsid: Tsid) -> Result<()> {
    let req = tokenstate::InitGlueSqlRequest {
        token_id: serialize(&my_token_id().await?)?,
        tsid: serialize(&tsid)?,
    };
    call(
        RegId::Static(NAME).inst(0),
        InitGlueSqlRequest(encode_protobuf(req)?),
    )
    .await?;

    let sql = String::from_utf8(include_bytes!("tables.sql").to_vec())?;
    exec_sql(tsid, sql).await
}

async fn exec_sql(tsid: Tsid, sql: String) -> Result<()> {
    let req = tokenstate::ExecGlueSqlRequest {
        token_id: serialize(&my_token_id().await?)?,
        sql,
        tsid: serialize(&tsid)?,
    };
    call(
        RegId::Static(NAME).inst(0),
        ExecGlueCmdRequest(encode_protobuf(req)?),
    )
    .await?;
    info!("SQL executed successfully.");

    Ok(())
}

fn parse_task(v: &Row) -> Result<Task> {
    Ok(Task {
        subject: sql_value_to_string(v.get_value_by_index(0).ok_or_err("subject")?)?.to_string(),
        creator: sql_value_to_string(v.get_value_by_index(1).ok_or_err("creator")?)?.parse()?,
        worker: sql_value_to_option_string(v.get_value_by_index(2).ok_or_err("worker")?)?
            .map(|v| v.parse())
            .transpose()?,
        status: sql_value_to_string(v.get_value_by_index(3).ok_or_err("status")?)?.parse()?,
        price: Balance::from_str_radix(
            &sql_value_to_string(v.get_value_by_index(4).ok_or_err("price")?)?,
            10,
        )?,
        required_deposit: Balance::from_str_radix(
            &sql_value_to_string(v.get_value_by_index(5).ok_or_err("required_deposit")?)?,
            10,
        )?,
    })
}
