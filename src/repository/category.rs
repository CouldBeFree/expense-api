extern crate dotenv;

use futures::StreamExt;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, Bson},
    results::{ InsertOneResult, DeleteResult },
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Collection, Database
};

use crate::models::category_model::Category;
use crate::utils::{UpdateType, Pagination, ArrayResponse, QueryParams, ParseStringToObjId};

use super::user_repo::UserRepo;

#[derive(Clone, Debug)]
pub struct CategoryRepo {
    pub collection: Collection<Category>
}

impl CategoryRepo {
    pub async fn init(db: &Database) -> Self {
        let collection: Collection<Category> = db.collection("category");
        CategoryRepo { collection }
    }

    pub async fn create_category(&self, category: Category, user_id: &String, user_repo: &UserRepo) -> Result<InsertOneResult, Error> {
        let owner_id = user_id.transform_to_obj_id().unwrap();
        let category = Category {
            id: None,
            owner: Some(owner_id),
            category_name: category.category_name
        };
        let category_result = self
            .collection
            .insert_one(category, None)
            .await
            .ok()
            .expect("Error creating category");
        user_repo.update_user_category(owner_id, &category_result.inserted_id, UpdateType::Add).await?;
        Ok(category_result)
    }
}
