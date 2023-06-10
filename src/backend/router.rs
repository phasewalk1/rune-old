use axum::{
    extract::{Extension, State, Path, RawQuery},
    http::{Request, header::HeaderMap},
    routing::{ get, post },
    response::{Response, IntoResponse},
    body::Body as AxumBody,
};
use crate::views::app::App;
use leptos::{ log, provide_context, view, };
use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context };
use leptos_router::RouteListing;
use sqlx::postgres::PgPool;
use sb_proto as pb;
use std::sync::Arc;
use super::state::AppState;

#[axum::debug_handler]
async fn leptos_gate(State(state): State<AppState>, req: Request<AxumBody>) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(state.opts.clone(),
        move |cx| {
            provide_context(cx, state.pool.clone());
        },
        |cx| view! { cx, <App/> }
    );
    handler(req).await.into_response()
}

pub async fn build_router(routes: Vec<RouteListing>, state: AppState, opt: leptos::LeptosOptions) -> axum::Router<()> {
    let app = axum::Router::new()
        .with_state(state)
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(opt.clone(), routes, |cx| view! { cx, <App/> })
        .fallback(crate::statics::handle_static)
        .layer(Extension(Arc::new(opt)));
    return app;
}
