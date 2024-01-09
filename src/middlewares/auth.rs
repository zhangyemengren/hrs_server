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
) -> Result<Response, (CookieJar, StatusCode)> {
    let url = request.uri().path().to_string();
    if WHITELIST.contains(&url.as_str()) {
        println!("命中白名单");
        let response = next.run(request).await;
        return Ok(response);
    }
    let Some(session) = jar_private.get("session") else {
        println!("cookies: {:?}", jar_private.get("session"));
        return Err((jar.remove("is_login"), StatusCode::UNAUTHORIZED));
    };
    let response = next.run(request).await;
    println!("cookies: {:?} {:?}", session, jar.get("is_login"));
    Ok(response)
}
