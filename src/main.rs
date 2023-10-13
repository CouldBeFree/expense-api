extern crate dotenv;

mod views;
mod db;
mod models;
mod repository;
mod app_state;

use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use actix_cors::Cors;
use env_logger::Env;
use repository::user_repo::UserRepo;
use app_state::app_state::AppState;
use db::db::DatabaseInstance;

// use crate::models::user_model::User;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db_instance = DatabaseInstance::init().await;
    let user_repo = UserRepo::init(&db_instance.instance).await;
    let state = AppState {
        user_repo
    };
    
    HttpServer::new(move || {
        // TODO: configure CORS
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();

        let app = App::new()
            .app_data(Data::new(state.clone()))
            .configure(views::views_factory).wrap(cors).wrap(Logger::new("%a %{User-Agent}i %r %s %D"));
            return app
    })
        .bind("127.0.0.1:5052")?
        .run()
        .await
}
