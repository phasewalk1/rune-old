use sqlx::postgres::PgPool;

pub(crate) async fn fill_pool() -> Result<PgPool, sqlx::Error> {
    use std::env::var as evar;
    use sqlx::postgres::PgPoolOptions;

    let url = evar("DATABASE_URL").expect("DATABASE_URL");
    let depth = 10usize;
    return PgPoolOptions::new()
        .max_connections(100)
        .connect(&url)
        .await;
}
