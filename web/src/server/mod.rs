use axum::Router;
use dioxus::prelude::*;
use dioxus_fullstack::ServeConfig;
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer};

pub mod session;

pub async fn start_server(app: fn() -> Element) {

    // Initialize the session manager
    let session_layer = {
        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let session_store = MemoryStore::default();
        SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)))
    };

    // build our application with some routes
    let app = Router::new()
        // Server side render the application, serve static assets, and register server functions
        .serve_dioxus_application(ServeConfig::new().unwrap(), app)
        .layer(session_layer);

    // serve the app using the address passed by the CLI
    let addr = dioxus_cli_config::fullstack_address_or_localhost();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    
}