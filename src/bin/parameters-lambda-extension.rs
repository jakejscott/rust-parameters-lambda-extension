use lambda_extension::{extension_fn, Error, LambdaEvent, NextEvent};

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
    println!("[parameters] Init");

    // TODO

    let func = extension_fn(parameters_extension);
    lambda_extension::run(func).await
}
