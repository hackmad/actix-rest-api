extern crate derive_more;
extern crate diesel;
extern crate env_logger;
#[cfg_attr(test, macro_use)]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sodiumoxide;

mod schema;
mod users;

use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::{r2d2::ConnectionManager, PgConnection};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").map_or("0.0.0.0".to_string(), |v| v);
    let port = env::var("PORT")
        .map_or("8000".to_string(), |v| v)
        .parse::<u16>()
        .unwrap();

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
            .service(web::scope("/api/v1").configure(users::routes::init_routes))
    })
    .bind((host, port))?
    .run()
    .await
}
