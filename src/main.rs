use axum::{response::{Html, Json}, routing::get, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/api/users", get(test_user))
        .route("/*all", get(catch_all));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn catch_all() -> Html<&'static str> {
    Html("<h1>404</h1>")
}
async fn test_user() -> Json<Value> {
    Json(json!({
        "data":[
            {
                "id": 1,
                "name": "John Doe",
            },
            {
                "id": 2,
                "name": "Miles Davis",
            },
        ],
        "total": 2,
    }))
}