use crate::State;
use crate::controllers;
use crate::models::user as models;

use sqlx::Pool;
use sqlx::postgres::Postgres;


pub async fn login(mut req: tide::Request<State>) -> tide::Result {
    let user_login: models::UserLogin = req.body_json().await?;
    let db_pool: Pool<Postgres> = req.state().db_pool.clone();
    let hash_string: &str = &req.state().pw_hash;
    let jwt_secret: &str = &req.state().jwt_secret;

    let row: Option<models::User> = controllers::user::get_by_username(
        String::from(&user_login.username),
        &db_pool,
    ).await?;

    let res: tide::Response = controllers::auth::validate_login(
        row,
        String::from(&user_login.password),
        hash_string,
        jwt_secret,
    ).await?;

    Ok(res)
}

pub async fn signup(mut req: tide::Request<State>) -> tide::Result {
    let user_signup: models::UserPartial = req.body_json().await?;
    let db_pool: Pool<Postgres> = req.state().db_pool.clone();
    let hash_string: &str = &req.state().pw_hash;
    let row: models::User = controllers::user::create(user_signup, &db_pool, hash_string).await?;

    let mut res: tide::Response = tide::Response::new(201);
    res.set_body(tide::Body::from_json(&row)?);

    Ok(res)
}
