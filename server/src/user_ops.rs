use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::models::{NewUser, User};

pub fn create_user(conn: &mut PgConnection, email: &str, name: &str, password: &str) -> Result<User, Error> {
    use crate::schema::users;

    let new_user = NewUser { email, name, password };

    return match diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn) {
        Ok(user) => Ok(user),
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    };
}

pub fn find_user(conn: &mut PgConnection, email: &str) -> Option<User> {
    use crate::schema::users::dsl::users;
    if let Ok(user) = users
        .find(email)
        .select(User::as_select())
        .first(conn) {
        Some(user)
    } else {
        None
    }
}

pub fn list_users(conn: &mut PgConnection) -> Vec<User> {
    use crate::schema::users::dsl::users;
    users
        .select(User::as_select())
        .load(conn)
        .expect("Error occurred during all user load")
}