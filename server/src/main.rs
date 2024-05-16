extern crate r2d2_postgres;

use std::io::Write;

use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_session::config::{BrowserSession, CookieContentSecurity};
use actix_web::{App, cookie::Key, HttpServer, web};
use actix_web::cookie::SameSite;
use actix_web::middleware::Logger;
use chrono::Local;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use env_logger::{Builder, Env};

use crate::handlers::auth::{login, logout, register_user};
use crate::handlers::resource::{assets, index};
use crate::handlers::user;

mod models;
mod schema;
mod user_ops;
mod redirect;
mod handlers;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct AppState {
    db: Pool<ConnectionManager<PgConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Builder::from_env(Env::default().default_filter_or("warn"))
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })

        .init();

    dotenv().ok();

    log::info!("Application is starting");

    let database_url = get_property("DATABASE_URL");
    log::info!("Configuring connection pool with database: {}", database_url);
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = Pool::builder()
        .max_size(10)
        .build(connection_manager)
        .expect("Cannot create a connection pool");

    log::info!("Running migrations");
    run_migration(&mut db_pool.get().expect("Cannot acquire DB connection"));

    log::info!("Starting http server");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db_pool.clone() }))
            .wrap(redirect::CheckLogin)
            .wrap(IdentityMiddleware::default())
            .wrap(init_session_middleware())
            .wrap(Logger::default())
            .service(
                web::scope("api")
                    .service(
                        web::scope("users")
                            .service(user::get_users)
                            .service(user::get_user)
                            .service(user::create_user)
                    )
            )
            .service(assets)
            .service(index)
            .service(login)
            .service(logout)
            .service(register_user)
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).expect("Migration execution failed");
}

fn get_property<'a>(property_name: &str) -> String {
    let error_message = format!("{} must be set", property_name);
    std::env::var(property_name).expect(&error_message)
}

fn init_session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
        CookieSessionStore::default(), Key::from(&[0; 64]),
    )
        .cookie_name(String::from("session"))
        .session_lifecycle(BrowserSession::default())
        .cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private)
        .cookie_http_only(true)
        .build()
}