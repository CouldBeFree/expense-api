use actix_web::{Responder, HttpResponse, web::{Data, Json, Path}};
use crate::{models::income_model::Income, app_state::app_state::AppState};
use crate::jwt::JwtToken;
use crate::utils::Error;

pub async fn update(db: Data<AppState>, new_income: Json<Income>, token: JwtToken, path: Path<String>) -> impl Responder {
    let user_id = token.user_id;
    let income_id = path.into_inner();
    let income_data = Income {
        id: None,
        owner: None,
        amount: new_income.amount.to_owned(),
        income_name: new_income.income_name.to_owned(),
        date: new_income.date.to_owned()
    };
    let res = db.income_repo.update_income(income_data, &income_id, &user_id).await;
    match res {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::BadRequest().json(Error{error: e.to_string()})
    }
}