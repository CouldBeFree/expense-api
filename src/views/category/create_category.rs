use actix_web::{Responder, HttpResponse, web::{Data, Path, Json}};
use crate::app_state::app_state::AppState;
use crate::jwt::JwtToken;
use crate::utils::Error;
use crate::models::category_model::Category;

pub async fn create(db: Data<AppState>, token: JwtToken, category: Json<Category>) -> impl Responder {
    let user_id = token.user_id;
    let category = Category {
        category_name: category.category_name.to_owned(),
        owner: None,
        id: None
    };
    let result = db.category_repo.create_category(category, &user_id, &db.user_repo).await;
    match result {
        Ok(inserted_result) => HttpResponse::Ok().json(inserted_result.inserted_id),
        Err(err) => {
            HttpResponse::BadRequest().json(Error{error: err.to_string()})
        }
    }
}