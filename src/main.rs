use hrs_server::startup::App;

#[tokio::main]
async fn main() {
    App::init_log();
    let app = App::new().serve;

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
