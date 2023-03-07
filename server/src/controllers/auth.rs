use crate::auth::{jwt, password};
use crate::models::user::{User, UserLoginResponse};


pub async fn validate_login(
    user: Option<User>,
    password: String,
    hash_string: &str,
    jwt_secret: &str,
) -> tide::Result {
    let res: tide::Response = match user {
        None => tide::Response::new(401),
        Some(user) => {
            let valid: bool = password::validate_password(hash_string, password);

            if valid {
                let claims = jwt::create_claims(String::from(&user.username));
                let tokens = UserLoginResponse {
                    access_token: jwt::encode_jwt_secret(&claims, jwt_secret)?,
                };

                let mut response = tide::Response::new(200);
                response.set_body(tide::Body::from_json(&tokens)?);
                response
            } else {
                tide::Response::new(401)
            }
        }
    };

    Ok(res)
}
