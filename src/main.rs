use anyhow::Result;
use hyper::Response;

async fn simulate_spawn() -> Result<(), std::io::Error> {
    println!("starting thread...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    println!("done!");
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let task = tokio::spawn(async { simulate_spawn().await });
    task.await.unwrap()
}
