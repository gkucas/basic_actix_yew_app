use actix_web::{get, HttpResponse, post, web};
use actix_web::web::{Json};
use diesel::result::{DatabaseErrorKind, Error};

use crate::{AppState, user_ops};
use crate::handlers::errors::UserError;
use crate::handlers::errors::UserError::{InternalError, ValidationError};
use crate::models::User;
use crate::user_ops::{find_user, list_users};

#[get("/")]
async fn get_users(data: web::Data<AppState>) -> Result<Json<Vec<User>>, actix_web::Error> {
    let connection = &mut data.db.get().expect("Cannot acquire DB connection");
    Ok(Json(list_users(connection)))
}

#[post("/")]
async fn create_user(data: web::Data<AppState>, user: Json<User>) -> Result<Json<User>, UserError> {
    let connection = &mut data.db.get().expect("Cannot acquire DB connection");
    return match user_ops::create_user(connection, &user.email, &user.name, &user.password) {
        Ok(user) => { Ok(Json(user)) }
        Err(e) => match e {
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => { Err(ValidationError { error: String::from("User already exists") }) }
            _ => Err(InternalError)
        }
    };
}

#[get("/{email}")]
async fn get_user(data: web::Data<AppState>, email: web::Path<String>) -> HttpResponse {
    let connection = &mut data.db.get().expect("Cannot acquire DB connection");
    match find_user(connection, &email.to_string()) {
        None => { HttpResponse::NotFound().finish() }
        Some(user) => {
            HttpResponse::Ok()
                .json(user)
        }
    }
}