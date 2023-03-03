use server;
use tide::prelude::*;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    tide::log::start();

    // env variables
    let port: String = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let db_url: String = std::env::var("DATABASE_URL")
        .expect("Please provide a `DATABASE_URL` environment variable.");

    // init app
    let app: tide::Server<server::State> = server::init_tide_server(&db_url).await;
    let mut listener = app.bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to establish a connection, an error occured when binding to the port.");

    // listen for requests
    for info in listener.info().iter() {
        println!("Rust HTTP server now listening on {}...", info);
    }
    listener.accept().await.unwrap();
}
