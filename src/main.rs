use axum::Router;
use std::net::SocketAddr;

use restful_api::routes::route::user_routes;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(user_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);
    axum::serve(
        tokio::net::TcpListener::bind("localhost:3000").await.unwrap(),
        app,
    )
        .await
        .unwrap();
}