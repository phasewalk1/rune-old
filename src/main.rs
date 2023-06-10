use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    fn logger_with(lvl: log::Level) -> () {
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

    static POOL_DEPTH: usize = 100;

    async fn app() -> () {
        use rune::views::app::App;
        use rune::backend;
        use axum::{ extract::Extension, routing::post, Router as AxRouter };
        use leptos::*;
        use leptos_axum::generate_route_list as routemap;

        logger_with(log::Level::Debug);
        let ( pool, state ) = backend::state::stateful_pool(POOL_DEPTH).await;
        let ( conf, opts, addr ) = load_configs!();
        let routes = routemap(|cx| view! { cx, <App/> }).await;

        let app = backend::router::build_router(routes, state, opts).await;
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
