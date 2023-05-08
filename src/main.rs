use lambda_runtime::{Error, LambdaEvent, service_fn};
use serde::{Deserialize, Serialize};
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
use tokio::runtime::Handle;
use tracing::event;


#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

pub(crate) async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let handle = Handle::current();

    futures::executor::block_on(async {
        handle
            .spawn(async {
                let command = event.payload.command;
                let resp = Response {
                    req_id: event.context.request_id,
                    msg: format!("{}", command),
                };
                Ok(resp)
            })
            .await
            .expect("Task spawned in Tokio executor panicked")
    })
}

//
// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     tracing_subscriber::fmt()
//         .with_max_level(tracing::Level::INFO)
//         .without_time()
//         .init();
//     let func = service_fn(my_handler);
//     lambda_runtime::run(func).await?;
//     Ok(())
// }

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {

    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb+srv://LambdaAccess:KHM6ZkuWDvkSDj6T@serverlessinstance0.rlw29ut.mongodb.net/?retryWrites=true&w=majority";
    let mut client_options = ClientOptions::parse(uri)?;
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;
    // Send a ping to confirm a successful connection
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    tokio::task::spawn_blocking(move || {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .without_time()
            .init();
        let func = service_fn(my_handler);
        lambda_runtime::run(func)
    })
        .await
        .expect("Blocking task panicked");

    Ok(())
}

