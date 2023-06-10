use axum::{
    extract::{Extension, State},
    http::Request,
};
use sb_proto as pb;
use std::sync::Arc;
use super::state::AppState;

pub(crate) async fn put_message(State(pool): State<Arc<AppState>>, req: Request<pb::MsgInTransit>) {
    
}
