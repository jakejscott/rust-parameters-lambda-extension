use lambda_extension::{extension_fn, Error, LambdaEvent, NextEvent};
use rust_parameters_lambda_extension::fetch_parameters;
use serde_json::json;

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("[parameters] init");

    let config = aws_config::load_from_env().await;
    let ssm: aws_sdk_ssm::Client = aws_sdk_ssm::Client::new(&config);
    let vars = std::env::vars().collect();
    let parameters = fetch_parameters(vars, &ssm).await.unwrap();

    println!("[parameters] fetched: {}", json!(parameters).to_string());

    // TODO: Start a web server and serve

    let func = extension_fn(parameters_extension);
    lambda_extension::run(func).await
}
