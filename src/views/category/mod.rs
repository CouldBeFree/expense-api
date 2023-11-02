mod create_category;

use actix_web::web::{ServiceConfig, post, scope};

pub fn category_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("category")
        .route("", post().to(create_category::create))
    );
}
