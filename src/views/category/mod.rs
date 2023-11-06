mod create_category;
mod get_categories;
mod remove_category;

use actix_web::web::{ServiceConfig, post, scope, get, delete};

pub fn category_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("category")
        .route("", post().to(create_category::create))
        .route("", get().to(get_categories::get_categories))
        .route("{id}", delete().to(remove_category::remove_category))
    );
}
