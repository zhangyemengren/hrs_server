use crate::response::{GenericBody, Status};
use axum::{
    extract,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

pub async fn auth(
    headers: HeaderMap,
    mut request: extract::Request,
    next: Next,
) -> Result<Response, Response> {
    let token = headers
        .get("Authorization")
        .ok_or_else(|| unauthorized_response())?
        .to_str()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let data = Jwt::default()
        .validate(token)
        .map_err(|_| unauthorized_response())?;
    // 修改extensions提取器值
    request.extensions_mut().insert(data.claims);
    let response = next.run(request).await;
    Ok(response)
}
// 生成未授权的响应
fn unauthorized_response() -> Response {
    let error_response = Json(GenericBody {
        status: Status::HttpError,
        msg: "Not Login".to_string(),
        data: (),
    });

    (StatusCode::UNAUTHORIZED, error_response).into_response()
}

/// jwt
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub(crate) secret: &'static [u8],
    pub(crate) claims: Claims,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) aud: String,
    pub(crate) sub: String,
    pub(crate) exp: u64,
    pub(crate) user: UserInfo,
    pub(crate) permission: Vec<Permission>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Permission {
    pub(crate) module_id: i32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserInfo {
    pub(crate) name: Option<String>,
    pub(crate) username: String,
    pub(crate) user_id: i32,
    pub(crate) role: i32,
}

impl Jwt {
    const COMMON_AUD: &'static str = "hrs";
    const COMMON_SUB: &'static str = "sso";
    const COMMON_SECRET: &'static [u8] = b"secret";

    pub fn new() -> Self {
        let claims = Claims::default();
        let secret = Self::COMMON_SECRET;
        Self { secret, claims }
    }
    pub fn aud(self, aud: &str) -> Self {
        let aud = aud.to_owned();
        Self {
            claims: Claims { aud, ..self.claims },
            ..self
        }
    }
    pub fn sub(self, sub: &str) -> Self {
        let aud = sub.to_owned();
        Self {
            claims: Claims { aud, ..self.claims },
            ..self
        }
    }
    pub fn exp(self, exp: u64) -> Self {
        Self {
            claims: Claims { exp, ..self.claims },
            ..self
        }
    }
    pub fn user(self, user: UserInfo) -> Self {
        Self {
            claims: Claims {
                user,
                ..self.claims
            },
            ..self
        }
    }
    pub fn permission(self, permission: Vec<Permission>) -> Self {
        Self {
            claims: Claims {
                permission,
                ..self.claims
            },
            ..self
        }
    }
    pub fn new_token(&self) -> anyhow::Result<String> {
        let r = jwt::encode(
            &jwt::Header::default(),
            &self.claims,
            &jwt::EncodingKey::from_secret(self.secret),
        )?;
        Ok(r)
    }
    pub fn validate(&self, token: &str) -> anyhow::Result<jwt::TokenData<Claims>> {
        let mut validation = jwt::Validation::default();
        validation.set_audience(&[self.claims.aud.to_owned()]);
        validation.sub = Some(self.claims.sub.to_owned());
        let r = jwt::decode::<Claims>(
            token,
            &jwt::DecodingKey::from_secret(self.secret),
            &validation,
        )?;
        Ok(r)
    }
}

impl Default for Jwt {
    fn default() -> Self {
        let mut j = Self::new();
        j.secret = Self::COMMON_SECRET;
        let utc_now = OffsetDateTime::now_utc();
        // 过期间隔
        let added_duration = Duration::minutes(30);
        let exp = (utc_now + added_duration).unix_timestamp();

        j.claims = Claims {
            aud: Self::COMMON_AUD.to_owned(),
            sub: Self::COMMON_SUB.to_owned(),
            exp: exp as u64,
            user: UserInfo::default(),
            permission: vec![],
        };
        j
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_test_jwt() {
        // 验证通过
        let j = Jwt::default();
        let token = j.new_token().unwrap();
        let data = j.validate(&token).unwrap();
        assert!(token.len() > 0);
        assert_eq!(data.claims.aud, "hrs");
        assert_eq!(data.claims.sub, "sso");
        // token过期
        let j = Jwt::default().exp(1);
        let token = j.new_token().unwrap();
        let data = j.validate(&token);
        let err = data
            .unwrap_err()
            .downcast::<jwt::errors::Error>()
            .unwrap()
            .into_kind();
        assert_eq!(err, jwt::errors::ErrorKind::ExpiredSignature);
        // aud不匹配
        let j = Jwt::default().aud("x");
        let token = j.new_token().unwrap();
        let data = Jwt::default().validate(&token);
        let err = data
            .unwrap_err()
            .downcast::<jwt::errors::Error>()
            .unwrap()
            .into_kind();
        assert_eq!(err, jwt::errors::ErrorKind::InvalidAudience);
    }
}
