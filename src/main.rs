extern crate dotenv;

use actix_web::{App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use env_logger::Env;

mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    HttpServer::new(|| {
        // TODO: configure CORS
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();

        let app = App::new()
            .configure(views::views_factory).wrap(cors).wrap(Logger::new("%a %{User-Agent}i %r %s %D"));
            return app
    })
        .bind("127.0.0.1:5052")?
        .run()
        .await
}
