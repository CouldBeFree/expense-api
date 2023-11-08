extern crate dotenv;

use futures::StreamExt;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, DeleteResult },
    options::{FindOptions, FindOneAndUpdateOptions, ReturnDocument},
    Collection, Database
};

use actix_web::web::Data;
use crate::models::expense_model::Expense;
use crate::utils::{UpdateType, ArrayResponse, ParseStringToObjId};
use crate::app_state::app_state::AppState;

use super::user_repo::UserRepo;

#[derive(Clone, Debug)]
pub struct ExpenseRepo {
    pub collection: Collection<Expense>
}

impl ExpenseRepo {
    pub async fn init(db: &Database) -> Self {
        let collection: Collection<Expense> = db.collection("expense");
        ExpenseRepo { collection }
    }

    pub async fn create_expense(&self, expense: Expense, state: &Data<AppState>) -> Result<InsertOneResult, Error> {
        let owner_id = expense.owner.to_owned().unwrap();
        let cat_id = expense.category_id.to_owned();
        let expense = Expense {
            id: None,
            date: expense.date,
            expense_name: expense.expense_name,
            owner: expense.owner.to_owned(),
            category_id: expense.category_id,
            amount: expense.amount
        };
        let expense_result = self
            .collection
            .insert_one(expense, None)
            .await
            .ok()
            .expect("Failed to insert expense");
        state.user_repo.update_user_expense(owner_id,  &expense_result.inserted_id, UpdateType::Add).await?;
        state.category_repo.update_category_expense(cat_id, owner_id, &expense_result.inserted_id, UpdateType::Add).await?;
        Ok(expense_result)
    }
}
