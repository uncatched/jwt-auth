#[macro_use]
extern crate diesel;

use actix_web::dev::ServiceRequest;
use actix_web::{App, HttpServer, Error};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::auth::validate_token;
use diesel::prelude::*;
use diesel:: r2d2::{self, ConnectionManager};

mod auth;
mod errors;
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
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(auth)
            .data(pool.clone())
            .service(handlers::users::get_all_users)
            .service(handlers::users::get_user_by_id)
            .service(handlers::users::add_user)
            .service(handlers::users::delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);

    match validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}