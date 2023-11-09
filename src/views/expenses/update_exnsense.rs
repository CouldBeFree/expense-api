use actix_web::{Responder, HttpResponse, web::{Data, Json, Path}};
use crate::{models::expense_model::Expense, app_state::app_state::AppState};
use crate::jwt::JwtToken;
use crate::utils::Error;

pub async fn update_expense(db: Data<AppState>, new_expense: Json<Expense>, token: JwtToken, path: Path<String>) -> impl Responder {
    let user_id = token.user_id;
    let expense_id = path.into_inner();
    let income_data = Expense {
        id: None,
        date: new_expense.date.to_owned(),
        amount: new_expense.amount,
        expense_name: new_expense.expense_name.to_owned(),
        owner: None,
        category_id: new_expense.category_id
    };
    let res = db.expense_repo.update_expense(income_data, &expense_id, user_id).await;
    match res {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::BadRequest().json(Error{error: e.to_string()})
    }
}