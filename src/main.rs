mod runner;

#[tokio::main]
async fn main() {
    runner::run();
    runner::async_run().await;
}
