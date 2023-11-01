extern crate dotenv;

use futures::StreamExt;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, Bson},
    results::{ InsertOneResult, DeleteResult },
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Collection, Database
};

use crate::models::income_model::Income;
use crate::utils::{UpdateType, Pagination, ArrayResponse, QueryParams, ParseStringToObjId};
use crate::utils::traits::user::UserRepository;

use super::user_repo::UserRepo;

impl ParseStringToObjId for String {
    fn transform_to_obj_id(&self) -> Result<ObjectId, mongodb::bson::oid::Error> {
        ObjectId::parse_str(self)
    }
}

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

    pub async fn get_incomes(&self, user_id: &String, query_params: QueryParams) -> Result<ArrayResponse<Income>, Error> {
        let user_obj_id = ObjectId::parse_str(user_id).unwrap();
        let filter_options = doc!{"owner": user_obj_id};
        let page = query_params.page;
        let per_page = query_params.per_page;
        let counted_skip = (page - 1) * per_page;
        let pagination_options = FindOptions::builder().limit(per_page as i64).skip(counted_skip as u64).build();
        let mut incomes = self
            .collection
            .find(filter_options.to_owned(), pagination_options)
            .await
            .unwrap();
        let mut results: Vec<Income> = Vec::new();
        while let Some(result) = incomes.next().await {
            match result {
                Ok(document) => {
                    results.push(document)
                },
                _ => return Err(Error::DeserializationError { message: "Deserilaztion error".to_string() })
            }
        }
        let filter_total = doc!{"owner": user_obj_id};
        let total_incomes_count = self
            .collection
            .count_documents(filter_total, None)
            .await
            .ok()
            .unwrap();
        let t = (total_incomes_count as f64) / (per_page as f64);
        let rounded_result = t.ceil() as u64;
        let counted_previous = page - 1;
        let prev: Option<usize> = if counted_previous == 0 { None } else {Some(counted_previous)};
        let pagination = Pagination {
            current: page,
            count: total_incomes_count,
            next: page + 1,
            pages: rounded_result,
            per_page: per_page,
            previous: prev
        };
        let response = ArrayResponse {
            data: results,
            pagination
        };
        Ok(response)
    }

    pub async fn remove_income(&self, user_id: &String, income_id: &String, user_repo: &UserRepo) -> Result<DeleteResult, Error> {
        let obj_income_id = income_id.transform_to_obj_id().unwrap();
        let user_obj_id = user_id.transform_to_obj_id().unwrap();
        let filter_options = doc!{"_id": obj_income_id, "owner": user_obj_id};
        let remove_result = self
            .collection
            .delete_one(filter_options, None)
            .await;
        match remove_result {
            Ok(dr) => {
                let bs = Bson::ObjectId(obj_income_id);
                user_repo.update_user_income(user_obj_id, &bs, UpdateType::Remove).await?;
                let count = dr.deleted_count;
                if count == 0 {
                    return Err(Error::DeserializationError { message: "Income not found".to_string() })
                }
                else {
                    return Ok(dr)
                }
            },
            Err(e) => Err(Error::DeserializationError { message: e.to_string() })
        }
    }
}