use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use sqlx::postgres::PgPool;
    use std::env::var as evar;

    fn instantiate_logger_with_level(lvl: log::Level) -> () {
        simple_logger::init_with_level(lvl).expect("couldn't initialize logger");
    }

    macro_rules! load_configs {
    () => {
        {
            let conf = leptos::get_configuration(None).await.unwrap();
            let opts = conf.clone().leptos_options;
            let addr = opts.site_addr;
            (conf, opts, addr)
        }
    };}


    async fn app() -> () {
        use rune::views::app::App;
        use rune::backend::{ router as rune_router, state as rune_state };
        use axum::{ extract::Extension, routing::post, Router as AxRouter };
        use leptos::*;
        use leptos_axum::generate_route_list as map_to_axum;

        instantiate_logger_with_level(log::Level::Debug);
        let ( pool, state ) = rune_state::instantiate_state().await;
        let ( conf, opts, addr ) = load_configs!();
        let routes = map_to_axum(|cx| view! { cx, <App/> }).await;

        let app = rune_router::build_router(routes, state, opts).await;
        log!("listening on http://{}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    app().await;
}
