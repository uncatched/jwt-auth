#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel:: r2d2::{self, ConnectionManager};

mod handlers;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(handlers::users::get_users)
            .service(handlers::users::get_user_by_id)
            .service(handlers::users::add_user)
            .service(handlers::users::delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
