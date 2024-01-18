use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{extract, Json};
use axum_extra::extract::{CookieJar, PrivateCookieJar};
use jsonwebtoken as jwt;
use time::{OffsetDateTime, UtcOffset};

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
/// jwt
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Jwt {
    secret: Vec<u8>,
    claims: Claims,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    aud: String,
    sub: String,
    exp: u64,
}

impl Jwt {
    pub fn new() -> Self {
        let claims = Claims::default();
        let secret = Vec::new();
        Self { secret, claims }
    }
    pub fn aud(&mut self, aud: &str) -> &mut Self {
        self.claims.aud = aud.to_owned();
        self
    }
    pub fn sub(&mut self, sub: &str) -> &mut Self {
        self.claims.sub = sub.to_owned();
        self
    }
    pub fn exp(&mut self, exp: u64) -> &mut Self {
        self.claims.exp = exp;
        self
    }
    pub fn new_token(&self) -> anyhow::Result<String> {
        let r = jwt::encode(
            &jwt::Header::default(),
            &self.claims,
            &jwt::EncodingKey::from_secret(self.secret.as_slice()),
        )?;
        Ok(r)
    }
    pub fn validate(&self, token: &str) -> anyhow::Result<jwt::TokenData<Claims>> {
        let mut validation = jwt::Validation::default();
        validation.set_audience(&[self.claims.aud.to_owned()]);
        validation.sub = Some(self.claims.sub.to_owned());
        let r = jwt::decode::<Claims>(
            token,
            &jwt::DecodingKey::from_secret(self.secret.as_slice()),
            &validation,
        )?;
        Ok(r)
    }
}

impl Default for Jwt {
    fn default() -> Self {
        let mut j = Self::new();
        j.secret = b"secret".to_vec();
        let utc_now = OffsetDateTime::now_utc();
        // 计算时区
        let offset = UtcOffset::from_hms(8, 0, 0).unwrap();
        // 加默认过期间隔
        let exp = utc_now.to_offset(offset).unix_timestamp() + 1;

        j.claims = Claims {
            aud: "hrs".to_owned(),
            sub: "sso".to_owned(),
            exp: exp as u64,
        };
        j
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jwt() {
        let j = Jwt::default();
        let token = j.new_token().unwrap();
        let data = j.validate(&token).unwrap();
        assert!(token.len() > 0);
        assert_eq!(data.claims.aud, "hrs");
        assert_eq!(data.claims.sub, "sso");
        // let j = Jwt::default().exp(1);
        // let token = j.new_token().unwrap();
        // let data = j.validate(&token).map_err(|e| e.to_string()).unwrap();
        // println!("{:?}", data);
    }
}
