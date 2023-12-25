use hrs_server::startup::App;

#[tokio::main]
async fn main() {
    App::new()
        .effect_with_log()
        .run()
        .await
        .expect("Run server failed");
}
