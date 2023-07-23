use std::io::Error;

use database::Database;
use indexer::indexer;
use serde_json::Value;
use server::server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let database = Database::new();
    let indexer_db = database.clone();

    let indexer_process = tokio::spawn(async { indexer(indexer_db).await });
    let server_process = tokio::spawn(async { server(database).await });

    // run all processes in parallel
    let processes = futures::future::join_all(vec![indexer_process, server_process]).await;

    for process in processes {
        if let Err(e) = &process {
            println!("join error {:?}", e);
        }

        if let Ok(Err(e)) = &process {
            println!("error in process {:?}", e);
        }
    }

    let em_types = reqwest::get("https://raw.githubusercontent.com/AudiusProject/audius-protocol/main/identity-service/relay-rate-limit.json").await.unwrap().json::<Value>().await.unwrap();
    let em_types = em_types.as_object().unwrap().keys().collect::<Vec<_>>();
    println!("{:?}", em_types);
    Ok(())
}
