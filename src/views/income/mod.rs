mod create_income;
mod update_income;
mod get_income;
mod get_incomes;
mod remove_income;

use actix_web::web::{ServiceConfig, post, scope, put, get, delete};

pub fn income_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("")
        .route("income", post().to(create_income::create))
        .route("income/{id}", get().to(get_income::get_income))
        .route("income/{id}", put().to(update_income::update))
        .route("incomes", get().to(get_incomes::get_incomes))
        .route("income/{id}", delete().to(remove_income::remove))
    );
}