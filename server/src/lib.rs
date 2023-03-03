pub mod controllers;
pub mod models;
pub mod views;

#[derive(Clone, Debug)]
pub struct State {
    pub db_pool: sqlx::postgres::PgPool,
}

impl State {
    pub async fn new(db_url: &str) -> Self {
        Self {
            db_pool: sqlx::postgres::PgPool::connect(db_url).await.unwrap(),
        }
    }
}

pub async fn init_tide_server(db_url: &str) -> tide::Server<State> {
    let state: State = State::new(db_url).await;
    let mut app: tide::Server<State> = tide::with_state(state);

    // middleware config
    app.with(
        tide::sessions::SessionMiddleware::new(
            tide::sessions::MemoryStore::new(),
            std::env::var("TIDE_SECRET")
                .expect("Please provide a `TIDE_SECRET` environment variable of at least 32 bytes.")
                .as_bytes(),
        )
    );

    // index views
    app.at("/").get(|_| async { Ok("Tide Demo API v0.1.0") });

    // movie views
    app.at("/movies")
        .get(views::movie::list)
        .post(views::movie::create);
    app.at("/movies/:id")
        .get(views::movie::get)
        .put(views::movie::update)
        .delete(views::movie::delete);

    // return app
    app
}
