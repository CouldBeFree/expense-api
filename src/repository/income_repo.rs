extern crate dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, Bson},
    results::{ InsertOneResult, UpdateResult, DeleteResult },
    Collection, Database
};

// use crate::models::user_model::{User, UserResponse, UserLogin};
use crate::models::income_model::Income;
use crate::utils::UpdateType;

use super::user_repo::UserRepo;

#[derive(Clone, Debug)]
pub struct IncomeRepo {
    pub collection: Collection<Income>
}

impl IncomeRepo {
    pub async fn init(db: &Database) -> Self {
        let collection: Collection<Income> = db.collection("income");
        IncomeRepo { collection }
    }

    pub async fn create_income(&self, income: Income, user_id: &String, user_repo: &UserRepo) -> Result<InsertOneResult, Error> {
        let obj_id = ObjectId::parse_str(user_id).unwrap();
        let new_income = Income {
            id: None,
            income_name: income.income_name.to_owned(),
            amount: income.amount.to_owned(),
            owner: Some(obj_id),
            date: income.date.to_owned()
        };
        let income_result = self
            .collection
            .insert_one(new_income, None)
            .await
            .ok()
            .expect("Error creating income");
        
        user_repo.update_user_income(obj_id, &income_result.inserted_id, UpdateType::Add).await?;
        Ok(income_result)
    }
}