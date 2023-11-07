mod create_category;
mod get_categories;
mod remove_category;
mod get_category;
mod update_category;

use actix_web::web::{ServiceConfig, post, scope, get, delete, put};

pub fn category_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("category")
        .route("", post().to(create_category::create))
        .route("", get().to(get_categories::get_categories))
        .route("{id}", delete().to(remove_category::remove_category))
        .route("{id}", get().to(get_category::get_category))
        .route("{id}", put().to(update_category::update_category))
    );
}
