use crate::State;
use crate::controllers::movie as controllers;
use crate::models::movie as models;

use sqlx::Pool;
use sqlx::postgres::Postgres;


pub async fn create(mut req: tide::Request<State>) -> tide::Result {
    let movie: models::MoviePartial = req.body_json().await?;
    let db_pool: Pool<Postgres> = req.state().db_pool.clone();
    let row: models::Movie = controllers::create(movie, &db_pool).await?;

    let mut res: tide::Response = tide::Response::new(201);
    res.set_body(tide::Body::from_json(&row)?);

    Ok(res)
}

pub async fn list(req: tide::Request<State>) -> tide::Result {
    let db_pool: Pool<Postgres> = req.state().db_pool.clone();
    let rows: Vec<models::Movie> = controllers::list(&db_pool).await?;

    let mut res: tide::Response = tide::Response::new(200);
    res.set_body(tide::Body::from_json(&rows)?);

    Ok(res)
}

pub async fn get(req: tide::Request<State>) -> tide::Result {
    let db_pool: Pool<Postgres> = req.state().db_pool.clone();
    let id: i32 = req.param("id")?.parse().unwrap();
    let row: Option<models::Movie> = controllers::get(id, &db_pool).await?;

    let res: tide::Response = match row {
        None => tide::Response::new(404),
        Some(row) => {
            let mut r: tide::Response = tide::Response::new(200);
            r.set_body(tide::Body::from_json(&row)?);
            r
        }
    };

    Ok(res)
}

pub async fn update(mut req: tide::Request<State>) -> tide::Result {
    let movie: models::Movie = req.body_json().await?;
    let db_pool: Pool<Postgres> = req.state().db_pool.clone();
    let id: i32 = req.param("id")?.parse().unwrap();
    let row: Option<models::Movie> = controllers::update(id, movie, &db_pool).await?;

    let res: tide::Response = match row {
        None => tide::Response::new(404),
        Some(row) => {
            let mut r: tide::Response = tide::Response::new(200);
            r.set_body(tide::Body::from_json(&row)?);
            r
        }
    };

    Ok(res)
}

pub async fn delete(req: tide::Request<State>) -> tide::Result {
    let db_pool: Pool<Postgres> = req.state().db_pool.clone();
    let id: i32 = req.param("id")?.parse().unwrap();
    let row: Option<()> = controllers::delete(id, &db_pool).await?;

    let res: tide::Response = match row {
        None => tide::Response::new(404),
        Some(_) => tide::Response::new(204),
    };

    Ok(res)
}
