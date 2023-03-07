use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::marker::PhantomData;
use tide::{Middleware, Next, Request, Response, StatusCode};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn invalid_auth_error(message: &str) -> tide::Response {
    let mut response: tide::Response = tide::Response::new(401);
    response.set_body(tide::Body::from(message));
    response
}

pub fn assert_claims(claims: &Claims) -> bool {
    let now_ts: i64 = Utc::now().timestamp();
    now_ts <= claims.exp
}

pub fn create_claims(username: String) -> Claims {
    let now: DateTime<Utc> = Utc::now();

    Claims {
        sub: username,
        iat: now.timestamp(),
        exp: now.checked_add_signed(Duration::hours(24)).unwrap().timestamp(),
    }
}

pub fn encode_jwt_secret<Claims: Serialize + DeserializeOwned + Send + Sync + 'static>(
    claims: &Claims,
    key: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_base64_secret(key)?,
    )
}

// JWT Auth Decoder and Auth Decoder Implementation
pub struct JwtAuthenticationDecoder<Claims: DeserializeOwned + Send + Sync + 'static> {
    validation: Validation,
    key: DecodingKey,
    _claims: PhantomData<Claims>,
}

impl<Claims: DeserializeOwned + Send + Sync + 'static> JwtAuthenticationDecoder<Claims> {
    pub fn default(key: DecodingKey) -> Self {
        Self::new(Validation::default(), key)
    }

    pub fn new(validation: Validation, key: DecodingKey) -> Self {
        Self {
            validation,
            key,
            _claims: PhantomData::default(),
        }
    }
}

#[async_trait]
impl<State, Claims> Middleware<State> for JwtAuthenticationDecoder<Claims>
where
    State: Clone + Send + Sync + 'static,
    Claims: DeserializeOwned + Send + Sync + 'static,
{
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> tide::Result {
        let header = req.header("Authorization");
        if header.is_none() {
            return Ok(next.run(req).await);
        }

        let values: Vec<_> = header.unwrap().into_iter().collect();

        if values.is_empty() {
            return Ok(next.run(req).await);
        }

        if values.len() > 1 {
            return Ok(Response::new(StatusCode::Unauthorized));
        }

        for value in values {
            let value = value.as_str();
            if !value.starts_with("Bearer") {
                continue;
            }

            let token = &value["Bearer ".len()..];
            println!("found authorization token: {token}");
            let data = match decode::<Claims>(token, &self.key, &self.validation) {
                Ok(c) => c,
                Err(_) => {
                    return Ok(Response::new(StatusCode::Unauthorized));
                }
            };

            req.set_ext(data.claims);
            break;
        }

        Ok(next.run(req).await)
    }
}
