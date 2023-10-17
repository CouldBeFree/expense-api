mod create_income;

use actix_web::web::{ServiceConfig, post, scope};

pub fn income_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("")
        .route("income", post().to(create_income::create))
    );
}