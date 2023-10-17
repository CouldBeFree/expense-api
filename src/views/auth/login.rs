use crate::{models::user_model::UserLogin, app_state::app_state::AppState};
use crate::jwt::JwtToken;

use actix_web::{Responder, HttpResponse, web::Data, web::Json};

pub async fn login(db: Data<AppState>, new_user: Json<UserLogin>) -> impl Responder {
    let data = UserLogin {
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };
    let is_valid_user = db.user_repo.login_user(data).await;
    match is_valid_user {
        Ok(user) => {
            let id = user.id.unwrap().to_string();
            let token = JwtToken::new(id);
            let raw_token = token.encode();
            HttpResponse::Ok().append_header(("token", raw_token)).take()
        },
        Err(_) => HttpResponse::Unauthorized().take()
    }
}