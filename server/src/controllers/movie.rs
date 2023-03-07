use crate::models::movie::{Movie, MoviePartial};

use sqlx::{query, query_as};
use sqlx::postgres::PgPool;


pub async fn create(movie: MoviePartial, db_pool: &PgPool) -> tide::Result<Movie> {
    let row: Movie = query_as!(
        Movie,
        r#"
        INSERT INTO movies (title, description) VALUES ($1, $2)
        RETURNING id as "id!", title as "title!", description as "description!"
        "#,
        movie.title,
        movie.description,
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| tide::Error::new(409, e))?;

    Ok(row)
}

pub async fn list(db_pool: &PgPool) -> tide::Result<Vec<Movie>> {
    let rows: Vec<Movie> = query_as!(Movie, r#"SELECT id, title, description from movies"#)
        .fetch_all(db_pool)
        .await
        .map_err(|e| tide::Error::new(409, e))?;

    Ok(rows)
}

pub async fn get(id: i32, db_pool: &PgPool) -> tide::Result<Option<Movie>> {
    let row: Option<Movie> = query_as!(
        Movie,
        r#"SELECT id, title, description from movies WHERE id = $1"#,
        id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| tide::Error::new(409, e))?;

    Ok(row)
}

pub async fn update(id: i32, movie: Movie, db_pool: &PgPool) -> tide::Result<Option<Movie>> {
    let row: Option<Movie> = query_as!(
        Movie,
        r#"
        UPDATE movies SET title = $2, description = $3
        WHERE id = $1
        returning id, title, description
        "#,
        id,
        movie.title,
        movie.description,
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| tide::Error::new(409, e))?;

    Ok(row)
}

pub async fn delete(id: i32, db_pool: &PgPool) -> tide::Result::<Option<()>> {
    let row = query!(r#"delete from movies WHERE id = $1 returning id"#, id)
        .fetch_optional(db_pool)
        .await
        .map_err(|e| tide::Error::new(409, e))?;

    Ok(match row {
        None => None,
        Some(_) => Some(()),
    })
}
