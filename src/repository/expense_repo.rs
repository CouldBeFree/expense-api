extern crate dotenv;

use futures::StreamExt;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, Bson},
    results::{ InsertOneResult, DeleteResult, UpdateResult },
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

    pub async fn update_expense(&self, expense: Expense, expense_id: &String, user_id: String) -> Result<Expense, Error> {
        let expense_obj_id = expense_id.transform_to_obj_id().unwrap();
        let user_obj_id = user_id.transform_to_obj_id().unwrap();
        let filter_options = doc!{"_id": expense_obj_id, "owner": user_obj_id};
        let amount = expense.amount;
        let bson_amount = Bson::from(amount as i64);
        let update_options = doc! {"$set": {"expense_name": expense.expense_name, "date": expense.date, "amount": bson_amount}};
        let opts = FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build();
        let update_result = self
            .collection
            .find_one_and_update(filter_options, update_options, opts)
            .await
            .unwrap();
        match update_result {
            Some(exp) => {
                Ok(exp)
            }
            None => Err(Error::DeserializationError { message: "Expense not found".to_string() })
        }
    }

    pub async fn remove_expense(&self, user_id: &String, expense_id: &String, state: &Data<AppState>) -> Result<DeleteResult, Error> {
        let user_obj_id = user_id.transform_to_obj_id().unwrap();
        let expense_obj_id = expense_id.transform_to_obj_id().unwrap();
        let filter_options = doc!{"_id": expense_obj_id, "owner": user_obj_id};
        let remove_result = self
            .collection
            .delete_one(filter_options, None)
            .await;
        match remove_result {
            Ok(dr) => {
                let bs = Bson::ObjectId(expense_obj_id);
                state.user_repo.update_user_expense(user_obj_id, &bs, UpdateType::Remove).await?;
                let count = dr.deleted_count;
                // state.category_repo.update_category_expense(category_id, user_id, expense_id, UpdateType::Remove).await?;
                if count == 0 {
                    return Err(Error::DeserializationError { message: "Expense not found".to_string() })
                }
                else {
                    return Ok(dr)
                }
            },
            Err(e) => Err(Error::DeserializationError { message: e.to_string() })
        }
    }
}
