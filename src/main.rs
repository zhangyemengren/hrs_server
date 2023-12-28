use hrs_server::startup::run;

#[tokio::main]
async fn main() {
    run().await.expect("Failed to run server");
}
