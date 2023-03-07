use server;
use tinyrand::{Rand, StdRand, Seeded};
use tinyrand_std::clock_seed::ClockSeed;

mod common;


// helper function(s) to get a partial struct(s)
fn generate_movie_partial(title: String, desc: String) -> server::models::movie::MoviePartial {
    server::models::movie::MoviePartial {
        title: Some(title),
        description: Some(desc),
    }
}

fn generate_user_partial(username: String) -> server::models::user::UserPartial {
    let seed = ClockSeed::default().next_u64();
    let mut rand = StdRand::seed(seed);
    let num = rand.next_u64();

    server::models::user::UserPartial {
        username: Some(format!("{}-{}", username, num)),
        first_name: Some("first".to_string()),
        last_name: Some("last".to_string()),
        password: Some("password".to_string()),
    }
}

async fn get_access_token(username: String) -> String {
    let state: server::State = server::State::new(
        std::env::var("DATABASE_URL").unwrap().as_str(),
        std::env::var("PASSWORD_HASH").unwrap().as_str(),
        std::env::var("SESSION_SECRET").unwrap().as_str(),
    ).await;
    let hash_string: &str = &state.pw_hash;
    let jwt_secret: &str = &state.jwt_secret;

    let user = generate_user_partial(username);
    let row = server::controllers::user::create(user, &state.db_pool.clone(), hash_string).await.unwrap();
    let claims = server::auth::jwt::create_claims(row.username);
    
    server::auth::jwt::encode_jwt_secret(&claims, jwt_secret).unwrap()
}

#[async_std::test]
async fn test_create_movie() -> tide::Result<()> {
    // arrange
    common::setup_vars();
    let app: tide::Server<server::State> = server::init_tide_server().await;
    let access_token = get_access_token("test_create_movie".to_string()).await;

    // act
    let movie = generate_movie_partial("create title".to_string(), "create desc".to_string());
    let res: surf::Response = surf::Client::with_http_client(app)
        .post("https://example.com/api/v1/movies")
        .body(serde_json::to_string(&movie)?)
        .header("Authorization", format!("Bearer {}", access_token))
        .await?;

    // assert
    assert_eq!(201, res.status());
    Ok(())
}

#[async_std::test]
async fn test_list_movies() -> tide::Result<()> {
    // arrange
    common::setup_vars();
    let app: tide::Server<server::State> = server::init_tide_server().await;
    let access_token = get_access_token("test_list_movies".to_string()).await;

    // act
    let res: surf::Response = surf::Client::with_http_client(app)
        .get("https://example.com/api/v1/movies")
        .header("Authorization", format!("Bearer {}", access_token))
        .await?;

    // assert
    assert_eq!(200, res.status());
    Ok(())
}

#[async_std::test]
async fn test_get_movie() -> tide::Result<()> {
    // arrange
    common::setup_vars();
    let app: tide::Server<server::State> = server::init_tide_server().await;
    let access_token = get_access_token("test_list_movies".to_string()).await;

    // act
    let movie = generate_movie_partial("get title".to_string(), "get desc".to_string());
    let row = server::controllers::movie::create(movie, &app.state().db_pool.clone()).await?;
    let res: surf::Response = surf::Client::with_http_client(app)
        .get(format!("https://example.com/api/v1/movies/{}", row.id))
        .header("Authorization", format!("Bearer {}", access_token))
        .await?;

    // assert
    assert_eq!(200, res.status());
    Ok(())
}

#[async_std::test]
async fn test_update_movie() -> tide::Result<()> {
    // arrange
    common::setup_vars();
    let app: tide::Server<server::State> = server::init_tide_server().await;
    let access_token = get_access_token("test_list_movies".to_string()).await;

    // act
    let movie = generate_movie_partial("update title".to_string(), "update desc".to_string());
    let row = server::controllers::movie::create(movie, &app.state().db_pool.clone()).await?;
    let res: surf::Response = surf::Client::with_http_client(app)
        .put(format!("https://example.com/api/v1/movies/{}", row.id))
        .body(serde_json::to_string(&row)?)
        .header("Authorization", format!("Bearer {}", access_token))
        .await?;

    // assert
    assert_eq!(200, res.status());
    Ok(())
}

#[async_std::test]
async fn test_delete_movie() -> tide::Result<()> {
    // arrange
    common::setup_vars();
    let app: tide::Server<server::State> = server::init_tide_server().await;
    let access_token = get_access_token("test_list_movies".to_string()).await;

    // act
    let movie = generate_movie_partial("delete title".to_string(), "delete desc".to_string());
    let row = server::controllers::movie::create(movie, &app.state().db_pool.clone()).await?;
    let res: surf::Response = surf::Client::with_http_client(app)
        .delete(format!("https://example.com/api/v1/movies/{}", row.id))
        .header("Authorization", format!("Bearer {}", access_token))
        .await?;

    // assert
    assert_eq!(204, res.status());
    Ok(())
}
