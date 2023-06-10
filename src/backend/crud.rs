use axum::{
    extract::{Extension, State},
    http::Request,
};
use sb_proto as pb;
use std::sync::Arc;
use super::state::AppState;
use sqlx::postgres::PgPool;
use leptos::*;

pub fn with_pool(cx: Scope) -> Result<PgPool, ServerFnError> {
    use_context::<PgPool>(cx)
        .ok_or("Pool missing from cx.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn register_server_funcs() -> () {
    _ = PutMessage::register();
}

#[derive(sqlx::FromRow, Clone)]
pub struct MsgRecord {
    id: u32,
    sender: String,
    recipient: String,
    body: String,
    timestamp: String,
}

impl Into<pb::Msg> for MsgRecord {
    fn into(self) -> pb::Msg {
        return pb::Msg {
            id: self.id.to_string(), 
            sender: self.sender,
            recipient: self.recipient, 
            body: self.body, 
            timestamp: self.timestamp,
        }
    }
}

#[server(PutMessage, "/api")]
pub(crate) async fn put_message(
    cx: Scope,
    sender: String,
    recipient: String,
    body: String,
) -> Result<pb::Msg, ServerFnError> {
    let pool = with_pool(cx)?;
    let tx = pool.begin().await.unwrap();
    let record = sqlx::query_file!("sql/send.sql", sender, recipient, body)
        .fetch_one(&pool)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    return Ok(pb::Msg {
        id: record.id.to_string(),
        sender: record.sender,
        recipient: record.recipient,
        body: record.body,
        timestamp: record.sent_at.to_string(),
    });
}
