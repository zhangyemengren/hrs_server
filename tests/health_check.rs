use hrs_server::startup;
use sqlx::PgPool;

async fn spawn_app() {
    let app = startup::App::new().serve;

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    // let _ =tokio::spawn(server);
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
}

#[sqlx::test]
async fn test_db() {
    spawn_app().await;
    let pool = PgPool::connect("postgres://postgres:qwer1234@localhost:5432/hrs")
        .await
        .unwrap();
    let rec = sqlx::query!("SELECT * FROM companies")
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("{:?}", rec);
    assert!(true);
}
