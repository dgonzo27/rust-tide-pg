use server;

mod common;


fn generate_movie_partial(title: String, desc: String) -> server::models::movie::MoviePartial {
    server::models::movie::MoviePartial {
        title: Some(title),
        description: Some(desc),
    }
}

#[async_std::test]
async fn test_create_movie() -> tide::Result<()> {
    // arrange
    common::setup_vars();
    let db_url: String = std::env::var("DATABASE_URL").unwrap();
    let app: tide::Server<server::State> = server::init_tide_server(&db_url).await;

    // act
    let movie = generate_movie_partial("create title".to_string(), "create desc".to_string());
    let res: surf::Response = surf::Client::with_http_client(app)
        .post("https://example.com/movies")
        .body(serde_json::to_string(&movie)?)
        .await?;

    // assert
    assert_eq!(201, res.status());
    Ok(())
}

#[async_std::test]
async fn test_list_movies() -> tide::Result<()> {
    // arrange
    common::setup_vars();
    let db_url: String = std::env::var("DATABASE_URL").unwrap();
    let app: tide::Server<server::State> = server::init_tide_server(&db_url).await;

    // act
    let res: surf::Response = surf::Client::with_http_client(app)
        .get("https://example.com/movies")
        .await?;

    // assert
    assert_eq!(200, res.status());
    Ok(())
}

#[async_std::test]
async fn test_get_movie() -> tide::Result<()> {
    // arrange
    common::setup_vars();
    let db_url: String = std::env::var("DATABASE_URL").unwrap();
    let app: tide::Server<server::State> = server::init_tide_server(&db_url).await;

    // act
    let movie = generate_movie_partial("get title".to_string(), "get desc".to_string());
    let row = server::controllers::movie::create(movie, &app.state().db_pool.clone()).await?;
    let res: surf::Response = surf::Client::with_http_client(app)
        .get(format!("https://example.com/movies/{}", row.id))
        .await?;

    // assert
    assert_eq!(200, res.status());
    Ok(())
}

#[async_std::test]
async fn test_update_movie() -> tide::Result<()> {
    // arrange
    common::setup_vars();
    let db_url: String = std::env::var("DATABASE_URL").unwrap();
    let app: tide::Server<server::State> = server::init_tide_server(&db_url).await;

    // act
    let movie = generate_movie_partial("update title".to_string(), "update desc".to_string());
    let row = server::controllers::movie::create(movie, &app.state().db_pool.clone()).await?;
    let res: surf::Response = surf::Client::with_http_client(app)
        .put(format!("https://example.com/movies/{}", row.id))
        .body(serde_json::to_string(&row)?)
        .await?;

    // assert
    assert_eq!(200, res.status());
    Ok(())
}

#[async_std::test]
async fn test_delete_movie() -> tide::Result<()> {
    // arrange
    common::setup_vars();
    let db_url: String = std::env::var("DATABASE_URL").unwrap();
    let app: tide::Server<server::State> = server::init_tide_server(&db_url).await;

    // act
    let movie = generate_movie_partial("delete title".to_string(), "delete desc".to_string());
    let row = server::controllers::movie::create(movie, &app.state().db_pool.clone()).await?;
    let res: surf::Response = surf::Client::with_http_client(app)
        .delete(format!("https://example.com/movies/{}", row.id))
        .await?;

    // assert
    assert_eq!(204, res.status());
    Ok(())
}
