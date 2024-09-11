use log::info;

mod utils;
mod handlers;
mod router;

#[tokio::main]
async fn main() {
    //loading env file
    dotenvy::dotenv().expect("Failed to load .env file");

    // Starting logger
    env_logger::builder().format_timestamp_millis().init();
    info!("Logger started");


    // Build the router
    let router = router::build_router();

    // run the app
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
