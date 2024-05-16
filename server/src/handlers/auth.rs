use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, Responder, web};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::web::Json;
use diesel::result::{DatabaseErrorKind, Error};

use crate::{AppState, user_ops};
use crate::handlers::errors::UserError;
use crate::handlers::errors::UserError::{InternalError, ValidationError};
use crate::models::User;
use crate::user_ops::find_user;

#[derive(serde::Deserialize)]
struct AuthRequest {
    pub username: String,
    pub password: String,
}

impl Responder for User {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/login")]
pub async fn login(data: web::Data<AppState>, request: HttpRequest, req_body: Json<AuthRequest>) -> impl Responder {
    let connection = &mut data.db.get().expect("Cannot acquire DB connection");
    let user = find_user(connection, &req_body.username.to_string());
    match user {
        Some(user) if user.password == req_body.password => {
            Identity::login(&request.extensions(), user.name.into()).unwrap();
            HttpResponse::Ok().finish()
        }
        _ => {
            HttpResponse::BadRequest()
                .body("Incorrect username or password")
        }
    }
}

#[post("/logout")]
pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

#[post("/register")]
async fn register_user(data: web::Data<AppState>, user: Json<User>) -> Result<Json<User>, UserError> {
    let connection = &mut data.db.get().expect("Cannot acquire DB connection");
    return match user_ops::create_user(connection, &user.email, &user.name, &user.password) {
        Ok(user) => { Ok(Json(user)) }
        Err(e) => match e {
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                Err(ValidationError { error: "User already exists".to_string() })
            }
            _ => Err(InternalError)
        }
    };
}