extern crate dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, Bson, Deserializer},
    results::{ InsertOneResult, UpdateResult, DeleteResult },
    options::{FindOneAndUpdateOptions, ReturnDocument},
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

    pub async fn update_income(&self, income: Income, income_id: &String, user_id: &String) -> Result<Income, Error> {
        let obj_id = ObjectId::parse_str(income_id).unwrap();
        let user_obj_id = ObjectId::parse_str(user_id).unwrap();
        let filter_options = doc!{"_id": obj_id, "owner": user_obj_id};
        let amount = income.amount;
        let bson_amount = Bson::from(amount as i64);
        let update_options = doc! {"$set": {"income_name": income.income_name, "date": income.date, "amount": bson_amount}};
        let opts = FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build();
        let update_result = self
            .collection
            .find_one_and_update(filter_options, update_options, opts)
            .await
            .unwrap();
        match update_result {
            Some(income) => {
                println!("Income name, {}", income.income_name);
                Ok(income)
            }
            None => Err(Error::DeserializationError { message: "Income not found".to_string() })
        }
    }

    pub async fn get_income(&self, income_id: &String, user_id: &String) -> Result<Income, Error> {
        let obj_income_id = ObjectId::parse_str(income_id).unwrap();
        let user_obj_id = ObjectId::parse_str(user_id).unwrap();
        let filter_options = doc!{"_id": obj_income_id, "owner": user_obj_id};
        let income = self
            .collection
            .find_one(filter_options, None)
            .await
            .unwrap();
        match income {
            Some(inc) => Ok(inc),
            None => Err(Error::DeserializationError { message: "Income not found".to_string() })
        }
    }
}