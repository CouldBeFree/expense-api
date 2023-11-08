use actix_web::{Responder, HttpResponse, web::{Data, Json, Path}};
use crate::{models::category_model::Category, app_state::app_state::AppState};
use crate::jwt::JwtToken;
use crate::utils::Error;

pub async fn update_category(db: Data<AppState>, new_category: Json<Category>, token: JwtToken, path: Path<String>) -> impl Responder {
    let user_id = token.user_id;
    let category_id = path.into_inner();
    let income_data = Category {
        id: new_category.id,
        owner: new_category.owner,
        category_name: new_category.category_name.to_owned(),
        expenses: None
    };
    let res = db.category_repo.update_category(&user_id, &category_id, income_data).await;
    match res {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::BadRequest().json(Error{error: e.to_string()})
    }
}