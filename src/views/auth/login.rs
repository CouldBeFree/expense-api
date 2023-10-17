use crate::{models::user_model::UserLogin, app_state::app_state::AppState};
use actix_web::{Responder, HttpResponse, web::Data, web::Json};
use crate::utils::Error;

pub async fn login(db: Data<AppState>, new_user: Json<UserLogin>) -> impl Responder {
    let data = UserLogin {
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };
    let is_valid_user = db.user_repo.login_user(data).await;
    match is_valid_user {
        Ok(()) => HttpResponse::Ok().json(Error{error: "Success".to_string()}),
        Err(e) => HttpResponse::BadRequest().json(Error{error: e.to_string()})
    }
}