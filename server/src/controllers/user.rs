use crate::auth::password::hash_password;
use crate::models::user::{User, UserPartial};

use sqlx::{query, query_as};
use sqlx::postgres::PgPool;


pub async fn create(user: UserPartial, db_pool: &PgPool, hash_string: &str) -> tide::Result<User> {
    let row: User = query_as!(
        User,
        r#"
        INSERT INTO users (username, first_name, last_name, password) VALUES 
        ($1, $2, $3, $4) RETURNING id, username, first_name, last_name, password, is_admin
        "#,
        user.username,
        user.first_name,
        user.last_name,
        hash_password(hash_string, user.password.unwrap()),
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| tide::Error::new(409, e))?;

    Ok(row)
}

pub async fn create_admin(user: UserPartial, db_pool: &PgPool, hash_string: &str) -> tide::Result<User> {
    let row: User = query_as!(
        User,
        r#"
        INSERT INTO users (username, first_name, last_name, password, is_admin) VALUES
        ($1, $2, $3, $4, $5) RETURNING id, username, first_name, last_name, password, is_admin
        "#,
        user.username,
        user.first_name,
        user.last_name,
        hash_password(hash_string, user.password.unwrap()),
        true,
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| tide::Error::new(409, e))?;

    Ok(row)
}

pub async fn list(db_pool: &PgPool) -> tide::Result<Vec<User>> {
    let rows: Vec<User> = query_as!(
        User,
        r#"
        SELECT id, username, first_name, last_name, password, is_admin
        from users
        "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| tide::Error::new(409, e))?;

    Ok(rows)
}

pub async fn get(id: i32, db_pool: &PgPool) -> tide::Result<Option<User>> {
    let row: Option<User> = query_as!(
        User,
        r#"
        SELECT id, username, first_name, last_name, password, is_admin
        from users WHERE id = $1
        "#,
        id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| tide::Error::new(409, e))?;

    Ok(row)
}

pub async fn get_by_username(username: String, db_pool: &PgPool) -> tide::Result<Option<User>> {
    let row: Option<User> = query_as!(
        User,
        r#"
        SELECT id, username, first_name, last_name, password, is_admin
        from users WHERE username = $1
        "#,
        username
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| tide::Error::new(409, e))?;

    Ok(row)
}

pub async fn update(id: i32, user: User, db_pool: &PgPool) -> tide::Result<Option<User>> {
    let row: Option<User> = query_as!(
        User,
        r#"
        UPDATE users SET first_name = $2, last_name = $3
        WHERE id = $1
        returning id, username, first_name, last_name, password, is_admin
        "#,
        id,
        user.first_name,
        user.last_name,
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| tide::Error::new(409, e))?;

    Ok(row)
}

pub async fn delete(id: i32, db_pool: &PgPool) -> tide::Result::<Option<()>> {
    let row = query!(
        r#"delete from users WHERE id = $1 returning id"#,
        id,
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| tide::Error::new(409, e))?;

    Ok(match row {
        None => None,
        Some(_) => Some(()),
    })
}
