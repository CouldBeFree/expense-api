use actix_web::{Responder, HttpResponse, web::Data, web::Json};
use crate::{models::income_model::Income, app_state::app_state::AppState};
use crate::jwt::JwtToken;
use crate::utils::Error;

pub async fn create(db: Data<AppState>, new_income: Json<Income>, token: JwtToken) -> impl Responder {
    let user_id = token.user_id;
    let income_data = Income {
        id: None,
        owner: None,
        amount: new_income.amount.to_owned(),
        income_name: new_income.income_name.to_owned(),
        date: new_income.date.to_owned()
    };
    let res = db.income_repo.create_income(income_data, &user_id, &db.user_repo).await;
    match res {
        Ok(res) => HttpResponse::Created().json(res.inserted_id),
        Err(e) => HttpResponse::BadRequest().json(Error{error: e.to_string()})
    }
}