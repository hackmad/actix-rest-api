use super::models::*;
use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, web, HttpResponse, Result,
};
use derive_more::{Display, Error};
use diesel::{r2d2::ConnectionManager, PgConnection};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    #[display(fmt = "Bad request.")]
    BadRequest { msg: String },
    #[display(fmt = "Forbidden.")]
    Forbidden { msg: String },
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest { msg: _ } => StatusCode::BAD_REQUEST,
            Self::Forbidden { msg: _ } => StatusCode::FORBIDDEN,
        }
    }
}

#[get("/users")]
pub async fn get_all(pool: web::Data<DbPool>) -> Result<web::Json<Vec<UserResponse>>, UserError> {
    let mut conn = pool.get().map_err(|_e| UserError::InternalError)?;
    Ok(web::Json(User::get_all_users(&mut conn)))
}

#[post("/users")]
pub async fn new_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUserRequest>,
) -> Result<web::Json<UserResponse>, UserError> {
    let mut conn = pool.get().map_err(|_e| UserError::InternalError)?;

    let username = new_user.username.clone();
    let result = User::insert_user(new_user.into_inner(), &mut conn);
    match result {
        Ok(_) => {
            let user = User::get_user_by_username(username.as_str(), &mut conn);
            match user {
                Some(u) => Ok(web::Json(u)),
                None => Err(UserError::BadRequest {
                    msg: "insert failed".to_string(),
                }),
            }
        }
        Err(e) => Err(UserError::BadRequest { msg: e.to_string() }),
    }
}

#[get("/users/{username}")]
pub async fn find_user(
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
) -> Result<Option<web::Json<UserResponse>>, UserError> {
    let mut conn = pool.get().map_err(|_e| UserError::InternalError)?;
    Ok(User::get_user_by_username(path.0.as_str(), &mut conn).map(|user| web::Json(user)))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    creds: web::Json<LoginRequest>,
) -> Result<web::Json<UserResponse>, UserError> {
    let mut conn = pool.get().map_err(|_e| UserError::InternalError)?;
    let result = User::login(creds.into_inner(), &mut conn);
    match result {
        Some(user) => Ok(web::Json(user)),
        None => Err(UserError::Forbidden {
            msg: "Invalid username and/or password".to_string(),
        }),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
        .service(new_user)
        .service(find_user)
        .service(login);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, web, App};
    use r2d2;
    use std::env;

    lazy_static! {
        static ref DB_POOL: DbPool = {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let manager = ConnectionManager::<PgConnection>::new(database_url);

            // Create connection pool
            r2d2::Pool::builder()
                .build(manager)
                .expect("Failed to create database pool.")
        };
    }

    #[actix_web::test]
    async fn get_all_users_returns_empty_when_no_users_exist() {
        let app = App::new()
            .app_data(web::Data::new(DB_POOL.clone()))
            .service(get_all);
        let mut app = test::init_service(app).await;

        let request = test::TestRequest::get().uri("/users").to_request();
        let response = test::call_service(&mut app, request).await;

        assert_eq!(response.status(), http::StatusCode::OK);

        let body = test::read_body(response).await;
        assert_eq!(body, "[]".as_bytes());
    }
}
