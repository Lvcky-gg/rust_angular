use axum::{
    // routing::get,
    Router,
};
// use std::net::SocketAddr;
use tower_http::{
    services::{ServeDir, ServeFile},
    // trace::TraceLayer,
};

#[tokio::main]
async fn main() {

    let serve_dir = ServeDir::new("dist/frontend/browser/")
        .not_found_service(ServeFile::new("./dist/frontend/browser/index.html"))
        .append_index_html_on_directories(true);

    let app = Router::new().route_service("/", serve_dir);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
