use axum::{
    extract::{Extension, State},
    http::Request,
    routing::post,
};
use crate::views::app::App;
use leptos::view;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos_router::RouteListing;
use sqlx::postgres::PgPool;
use sb_proto as pb;
use std::sync::Arc;
use super::state::AppState;

pub async fn build_router(routes: Vec<RouteListing>, state: AppState, opt: leptos::LeptosOptions) -> axum::Router<()> {
    let app = axum::Router::new()
        .with_state(state)
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(opt.clone(), routes, |cx| view! { cx, <App/> })
        .fallback(crate::statics::handle_static)
        .layer(Extension(Arc::new(opt)));
    return app;
}
