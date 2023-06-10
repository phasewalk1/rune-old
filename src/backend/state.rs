use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct AppState(pub PgPool);

pub async fn stateful_pool(depth: usize) -> ( PgPool, super::state::AppState ) {
    if let Ok(pool) = super::pool::fill_pool(depth).await {
        return (
            pool.clone(),
            AppState(pool),
        );
    } else {
        panic!("Failed to instantiate application state");
    }
}
