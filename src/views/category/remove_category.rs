use actix_web::{Responder, HttpResponse, web::{Data, Path}};
use crate::app_state::app_state::AppState;
use crate::jwt::JwtToken;
use crate::utils::{Error, Success};

pub async fn remove_category(db: Data<AppState>, token: JwtToken, path: Path<String>) -> impl Responder {
    let user_id = token.user_id;
    let income_id = path.into_inner();
    let res = db.category_repo.remove_category(&user_id, &income_id, &db.user_repo).await;
    match res {
        Ok(_) => HttpResponse::Ok().json(Success{success: "Removed".to_owned()}),
        Err(e) => HttpResponse::BadRequest().json(Error{error: e.to_string()})
    }
}