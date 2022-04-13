mod api;
mod cli;
mod proto;

#[tokio::main]
async fn main() {
    cli::init().await;
}
