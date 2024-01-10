use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{extract, Json};
use axum_extra::extract::{CookieJar, PrivateCookieJar};

const WHITELIST: [&str; 3] = ["/", "/health_check", "/api/v1/login"];

#[derive(serde::Serialize)]
struct ErrorRes {
    code: i32,
    msg: String,
}
pub async fn auth(
    jar_private: PrivateCookieJar,
    jar: CookieJar,
    request: extract::Request,
    next: Next,
) -> Result<Response, Response> {
    let url = request.uri().path().to_string();
    if WHITELIST.contains(&url.as_str()) {
        println!("命中白名单");
        let response = next.run(request).await;
        return Ok(response);
    }
    let Some(session) = jar_private.get("session") else {
        println!("cookies: {:?}", jar_private.get("session"));
        return Err((
            StatusCode::UNAUTHORIZED,
            jar.remove("is_login"),
            Json(ErrorRes {
                code: 401,
                msg: "未登录".to_string(),
            }),
        )
            .into_response());
    };
    let response = next.run(request).await;
    println!("cookies: {:?} {:?}", session, jar.get("is_login"));
    Ok(response)
}
