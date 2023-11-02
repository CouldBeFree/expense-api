mod create_income;
mod update_income;
mod get_income;
mod get_incomes;
mod remove_income;

use actix_web::web::{ServiceConfig, post, scope, put, get, delete};

pub fn income_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("income")
        .route("", post().to(create_income::create))
        .route("{id}", get().to(get_income::get_income))
        .route("{id}", put().to(update_income::update))
        .route("", get().to(get_incomes::get_incomes))
        .route("{id}", delete().to(remove_income::remove))
    );
}