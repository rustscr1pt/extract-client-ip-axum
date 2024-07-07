use axum::{routing::get, Router};
use axum_client_ip::{InsecureClientIp, SecureClientIp, SecureClientIpSource};
use std::net::SocketAddr;

async fn handler(insecure_ip: InsecureClientIp, secure_ip: SecureClientIp) -> String {
    format!("{insecure_ip:?} {secure_ip:?}")
}

// Source of example : https://crates.io/crates/axum-client-ip/0.6.0

#[tokio::main]
async fn main() {
    async fn handler(insecure_ip: InsecureClientIp, secure_ip: SecureClientIp) -> String {
        format!("{insecure_ip:?} {secure_ip:?}")
    }

    let app = Router::new().route("/", get(handler))
        .layer(SecureClientIpSource::ConnectInfo.into_extension());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(
        listener,
        // Don't forget to add `ConnectInfo` if you aren't behind a proxy
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .await
        .unwrap()
}