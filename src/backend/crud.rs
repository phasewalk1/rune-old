use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use leptos::*;
    pub(crate) use sqlx::postgres::PgPool;

    use sb_proto as pb;

    #[server(SendMessage, "/api")]
    pub(crate) async fn send_message(
        cx: Scope,
        msg: pb::MsgInTransit,
    ) -> Result<(), ServerFnError> {
        return Ok(());
    }
}}
