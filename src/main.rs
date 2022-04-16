mod api;
mod cli;
mod conf;
mod proto;
mod storage;

#[tokio::main]
async fn main() {
    cli::init().await;
}
