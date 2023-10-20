use actix_web::{Responder, HttpResponse, HttpRequest, web::{Data, Query}};

use crate::app_state::app_state::AppState;
use crate::jwt::JwtToken;
use crate::utils::Error;
use crate::utils::QueryParams;

use std::collections::HashMap;

pub async fn get_incomes(req: HttpRequest, db: Data<AppState>, token: JwtToken) -> impl Responder {
    let params = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let per_page = params.get("per_page");
    let page = params.get("page");
    let query_params = QueryParams::new(per_page, page);
    let user_id = token.user_id;
    let res = db.income_repo.get_incomes(&user_id, query_params).await;
    match res {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::NotFound().json(Error{error: e.to_string()})
    }
}