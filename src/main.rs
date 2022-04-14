mod api;
mod cli;
mod conf;
mod proto;

#[tokio::main]
async fn main() {
    cli::init().await;
}
