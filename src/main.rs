extern crate derive_more;
#[macro_use]
extern crate diesel;
extern crate env_logger;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sodiumoxide;

mod models;
mod routes;
mod schema;

use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::{r2d2::ConnectionManager, PgConnection};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Create connection pool
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v1")
                    .service(routes::get_all)
                    .service(routes::new_user)
                    .service(routes::find_user)
                    .service(routes::login),
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
