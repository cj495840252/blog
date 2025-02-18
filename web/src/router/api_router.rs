#[cfg(feature = "server")]
pub fn get_api_router() -> axum::Router {
    use axum::routing::get;
    use tracing::info;
    axum::Router::new().route(
        "/download",
        get(|| async {
            info!("access /download");
            "download, World!"
        }),
    )
}
