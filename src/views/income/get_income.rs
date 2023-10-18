use actix_web::{Responder, HttpResponse, web::{Data, Path}};
use crate::app_state::app_state::AppState;
use crate::jwt::JwtToken;
use crate::utils::Error;

pub async fn get_income(db: Data<AppState>, token: JwtToken, path: Path<String>) -> impl Responder {
    let user_id = token.user_id;
    let id = path.into_inner();
    let res = db.income_repo.get_income(&id, &user_id).await;
    match res {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::NotFound().json(Error{error: e.to_string()})
    }
}