use sqlx::postgres::PgPool;

pub(crate) async fn fill_pool(depth: usize) -> Result<PgPool, sqlx::Error> {
    use std::env::var as evar;
    use sqlx::postgres::PgPoolOptions;

    let url = evar("DATABASE_URL").expect("DATABASE_URL");
    return PgPoolOptions::new()
        .max_connections(depth as u32)
        .connect(&url)
        .await;
}
