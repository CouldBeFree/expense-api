use crate::{models::user_model::User, app_state::app_state::AppState};
use actix_web::{Responder, HttpResponse, web::Data, web::{Json, Path}};

pub async fn register(db: Data<AppState>, new_user: Json<User>) -> impl Responder {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        expenses: Some(vec![]),
        incomes: Some(vec![])
    };
    let result = db.user_repo.create_user(data).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            HttpResponse::BadRequest().json(err.to_string())
        }
    }
}