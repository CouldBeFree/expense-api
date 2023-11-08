use actix_web::{Responder, HttpResponse, web::Data, web::Json};
use crate::{models::expense_model::Expense, app_state::app_state::AppState};
use crate::jwt::JwtToken;
use crate::utils::{Error, ParseStringToObjId};

pub async fn create_expense(db: Data<AppState>, new_expense: Json<Expense>, token: JwtToken) -> impl Responder {
    let user_id = token.user_id.transform_to_obj_id().unwrap();
    let income_data = Expense {
        id: None,
        owner: Some(user_id),
        date: new_expense.date.to_owned(),
        expense_name: new_expense.expense_name.to_owned(),
        category_id: new_expense.category_id,
        amount: new_expense.amount
    };
    let res = db.expense_repo.create_expense(income_data, &db).await;
    match res {
        Ok(res) => HttpResponse::Created().json(res.inserted_id),
        Err(e) => HttpResponse::BadRequest().json(Error{error: e.to_string()})
    }
}