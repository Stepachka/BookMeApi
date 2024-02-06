use std::env;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2::Pool;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<ConnectionManager<PgConnection>>
}

impl AppState {
    pub fn new() -> Pool<ConnectionManager<PgConnection>> {
        dotenv().ok();
        let database_url    = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager         = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool")
    }


}