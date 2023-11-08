mod create_expense;

use actix_web::web::{ServiceConfig, post, scope, put, get, delete};

pub fn expense_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("expense")
        .route("", post().to(create_expense::create_expense))
    );
}