mod register;
mod login;

use actix_web::web::{ServiceConfig, post, scope};

pub fn auth_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("auth")
        .route("register", post().to(register::register))
        .route("login", post().to(login::login))
    );
}