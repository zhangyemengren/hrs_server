use hrs_server::startup::App;

#[tokio::main]
async fn main() {
    App::init_log();
    let App { app, config } = App::new();
    println!("{:?}", config);
    // run it
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await
    .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
