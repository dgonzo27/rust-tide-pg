pub mod auth;
pub mod controllers;
pub mod models;
pub mod views;


#[derive(Clone, Debug)]
pub struct State {
    pub db_pool: sqlx::postgres::PgPool,
    pub pw_hash: String,
    pub jwt_secret: String,
}

impl State {
    pub async fn new(db_url: &str, pw_hash: &str, jwt_secret: &str) -> Self {
        Self {
            db_pool: sqlx::postgres::PgPool::connect(db_url).await.unwrap(),
            pw_hash: String::from(pw_hash),
            jwt_secret: String::from(jwt_secret),
        }
    }
}


pub async fn init_tide_server() -> tide::Server<State> {
    let state: State = State::new(
        std::env::var("DATABASE_URL").unwrap().as_str(),
        std::env::var("PASSWORD_HASH").unwrap().as_str(),
        std::env::var("SESSION_SECRET").unwrap().as_str(),
    ).await;
    let mut app: tide::Server<State> = tide::with_state(state);
    
    // middleware config
    app.with(
        tide::sessions::SessionMiddleware::new(
            tide::sessions::MemoryStore::new(),
            std::env::var("TIDE_SECRET").unwrap().as_bytes(),
        )
    );
    app.with(auth::jwt::JwtAuthenticationDecoder::<auth::jwt::Claims>::new(
        jsonwebtoken::Validation::default(),
        jsonwebtoken::DecodingKey::from_base64_secret(
            std::env::var("SESSION_SECRET").unwrap().as_str()
        ).unwrap(),
    ));

    // index views
    app.at("/").get(|_| async { Ok("Tide Demo API v0.1.0") });

    // auth views
    app.at("/api/v1/auth/login").post(views::auth::login);
    app.at("/api/v1/auth/signup").post(views::auth::signup);

    // movie views
    app.at("/api/v1/movies")
        .get(views::movie::list)
        .post(views::movie::create);
    app.at("/api/v1/movies/:id")
        .get(views::movie::get)
        .put(views::movie::update)
        .delete(views::movie::delete);

    // return app
    app
}
