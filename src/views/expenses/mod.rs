mod create_expense;
mod update_exnsense;
mod remove_expense;

use actix_web::web::{ServiceConfig, post, scope, put, get, delete};

pub fn expense_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("expense")
        .route("", post().to(create_expense::create_expense))
        .route("{id}", put().to(update_exnsense::update_expense))
        .route("{id}", delete().to(remove_expense::remove_expense))
    );
}