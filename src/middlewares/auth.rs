use axum::extract;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::extract::{CookieJar, PrivateCookieJar};

const WHITELIST: [&str; 1] = ["/api/v1/login"];

pub async fn auth(
    jar_private: PrivateCookieJar,
    jar: CookieJar,
    request: extract::Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let url = request.uri().path();
    if WHITELIST.contains(&url) {
        println!("命中白名单");
    }
    let response = next.run(request).await;
    let data = jar_private.get("session");
    let data2 = jar.get("is_login");
    println!("cookies: {:?} {:?}", data, data2);
    Ok(response)
    // Err(StatusCode::UNAUTHORIZED)
}
