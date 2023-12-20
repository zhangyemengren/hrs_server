use hrs_server::{configuration::get_config, startup::App};

#[tokio::main]
async fn main() {
    App::init_log();
    let app = App::new().serve;
    get_config();
    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
