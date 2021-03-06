mod api;
mod cli;
mod conf;
mod frontend;
mod models;
mod proto;
mod storage;

#[tokio::main]
async fn main() {
    cli::init().await;
}
