use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, routing::get, Json, Router,
    Server,
};
use lambda_extension::{extension_fn, Error, LambdaEvent, NextEvent};
use rust_parameters_lambda_extension::{fetch_parameters, Parameter};
use serde_json::json;
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tokio::spawn;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

type Db = Arc<RwLock<Vec<Parameter>>>;

async fn parameters_extension(event: LambdaEvent) -> Result<(), Error> {
    match event.next {
        NextEvent::Shutdown(_e) => {
            println!("[parameters] Shutdown");
        }
        NextEvent::Invoke(_e) => {
            println!("[parameters] Invoke event");
        }
    }
    Ok(())
}

async fn index_handler(Extension(db): Extension<Db>) -> impl IntoResponse {
    let parameters = db.read().unwrap().clone();
    (StatusCode::OK, Json(parameters))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("[parameters] init");

    let config = aws_config::load_from_env().await;
    let ssm: aws_sdk_ssm::Client = aws_sdk_ssm::Client::new(&config);
    let vars = std::env::vars().collect();

    let parameters = fetch_parameters(vars, &ssm).await.unwrap();
    println!("[parameters] fetched: {}", json!(&parameters).to_string());

    let db = Db::new(RwLock::new(parameters));

    // Start a web server on a background task
    spawn(async {
        let app = Router::new().route("/", get(index_handler)).layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(db))
                .into_inner(),
        );

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        println!("[parameters] listening on: {}", addr);

        Server::bind(&addr).serve(app.into_make_service()).await
    });

    let func = extension_fn(parameters_extension);
    lambda_extension::run(func).await
}
