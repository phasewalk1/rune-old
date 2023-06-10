use axum::extract::FromRef;
use sqlx::postgres::PgPool;
use leptos::LeptosOptions;

#[derive(FromRef, Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub opts: LeptosOptions,
}

pub async fn stateful_pool(opts: LeptosOptions, depth: usize) -> ( PgPool, super::state::AppState ) {
    if let Ok(pool) = super::pool::fill_pool(depth).await {
        return (
            pool.clone(),
            AppState { pool, opts },
        );
    } else {
        panic!("Failed to instantiate application state");
    }
}
