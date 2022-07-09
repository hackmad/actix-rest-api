use super::schema::users;
use super::schema::users::dsl::users as all_users;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use sodiumoxide::crypto::pwhash::argon2id13;

/// User DTO
#[derive(Serialize, Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub first_name: String,
}

impl User {
    pub fn get_all_users(conn: &PgConnection) -> Vec<UserResponse> {
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("error!")
            .iter()
            .map(UserResponse::from)
            .collect()
    }

    pub fn insert_user(user: NewUserRequest, conn: &PgConnection) -> QueryResult<usize> {
        let new_user = NewUser::from(user);
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
    }

    pub fn get_user_by_username(username: &str, conn: &PgConnection) -> Option<UserResponse> {
        let users = all_users
            .filter(users::username.eq(username))
            .load::<User>(conn)
            .expect("error!");
        let n = users.len();
        if n == 0 {
            None
        } else {
            Some(UserResponse::from(&users[0]))
        }
    }

    pub fn login(creds: LoginRequest, conn: &PgConnection) -> Option<UserResponse> {
        let users = all_users
            .filter(users::username.eq(creds.username.clone()))
            .load::<User>(conn)
            .expect("error!");
        let n = users.len();
        if n == 0 {
            None
        } else {
            let mut padded = [0u8; 128];
            users[0]
                .password_hash
                .clone()
                .as_bytes()
                .iter()
                .enumerate()
                .for_each(|(i, val)| {
                    padded[i] = val.clone();
                });
            argon2id13::HashedPassword::from_slice(&padded)
                .map(|hp| argon2id13::pwhash_verify(&hp, creds.password.as_bytes()))
                .map(|valid| valid.then(|| UserResponse::from(&users[0])))
                .flatten()
        }
    }
}

/// Used to insert users to database
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
    pub first_name: String,
}

impl From<NewUserRequest> for NewUser {
    fn from(req: NewUserRequest) -> Self {
        let password_bytes = req.password.as_bytes();
        let hash = argon2id13::pwhash(
            password_bytes,
            argon2id13::OPSLIMIT_INTERACTIVE,
            argon2id13::MEMLIMIT_INTERACTIVE,
        )
        .unwrap();

        let texthash = std::str::from_utf8(&hash.0)
            .unwrap()
            .trim_end_matches('\u{0}')
            .to_string();

        Self {
            username: req.username,
            first_name: req.first_name,
            password_hash: texthash,
        }
    }
}

/// Used to get new user request
#[derive(Serialize, Deserialize, Clone)]
pub struct NewUserRequest {
    pub username: String,
    pub password: String,
    pub first_name: String,
}

/// Send user response back
#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub username: String,
    pub first_name: String,
}
impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        Self {
            username: user.username.clone(),
            first_name: user.first_name.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
