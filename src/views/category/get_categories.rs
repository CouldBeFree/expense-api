use actix_web::{Responder, HttpResponse, web::Data};
use crate::app_state::app_state::AppState;
use crate::jwt::JwtToken;
use crate::utils::Error;

pub async fn get_categories(db: Data<AppState>, token: JwtToken) -> impl Responder {
    let user_id = token.user_id;
    let result = db.category_repo.get_categories(&user_id).await;
    match result {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(err) => {
            HttpResponse::BadRequest().json(Error{error: err.to_string()})
        }
    }
}