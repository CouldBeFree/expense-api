extern crate dotenv;

use futures::StreamExt;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, Bson},
    results::{ InsertOneResult, DeleteResult },
    options::{FindOptions, FindOneAndUpdateOptions, ReturnDocument},
    Collection, Database
};

use crate::models::category_model::{Category, CategoryArrayResponse};
use crate::utils::{UpdateType, ArrayResponse, ParseStringToObjId};

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

    pub async fn get_category_by_id(&self, user_id: &String, category_id: &String) -> Result<CategoryArrayResponse, Error> {
        let user_obj_id = user_id.transform_to_obj_id().unwrap();
        let category_obj_id = category_id.transform_to_obj_id().unwrap();
        let filter_options = doc!{"_id": category_obj_id, "owner": user_obj_id};
        let category = self
            .collection
            .find_one(filter_options, None)
            .await
            .unwrap();
        match category {
            Some(cat) => {
                let response = CategoryArrayResponse {
                    id: cat.id,
                    category_name: cat.category_name
                };
                Ok(response)
            },
            None => Err(Error::DeserializationError { message: "Income not found".to_string() })
        }
    }

    pub async fn update_category(&self, user_id: &String, category_id: &String, category: Category) -> Result<CategoryArrayResponse, Error> {
        let user_obj_id = user_id.transform_to_obj_id().unwrap();
        let category_obj_id = category_id.transform_to_obj_id().unwrap();
        let filter_options = doc!{"_id": category_obj_id, "owner": user_obj_id};
        let opts = FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build();
        let update_options = doc! {"$set": {"category_name": category.category_name}};
        let update_result = self
            .collection
            .find_one_and_update(filter_options, update_options, opts)
            .await
            .unwrap();
        match update_result {
            Some(cat) => {
                let response = CategoryArrayResponse {
                    id: cat.id,
                    category_name: cat.category_name
                };
                Ok(response)
            }
            None => Err(Error::DeserializationError { message: "Category not found".to_string() })
        }
    }

    async fn get_category_by_name(&self, category_name: &String, owner_id: ObjectId) -> Result<Category, Error> {
        let filter_options = doc! {"category_name": category_name, "owner": owner_id};
        let category = self
            .collection
            .find_one(filter_options, None)
            .await
            .ok()
            .unwrap();
        match category {
            Some(cat) => Ok(cat),
            None => Err(Error::DeserializationError { message: "No category found".to_string() })
        }
    }

    pub async fn remove_category(&self, user_id: &String, category_id: &String, user_repo: &UserRepo) -> Result<DeleteResult, Error> {
        let obj_category_id = category_id.transform_to_obj_id().unwrap();
        let obj_user_id = user_id.transform_to_obj_id().unwrap();
        let filter_options = doc!{"_id": obj_category_id, "owner": obj_user_id};
        let remove_result = self
            .collection
            .delete_one(filter_options, None)
            .await;
        match remove_result {
            Ok(dr) => {
                let bs = Bson::ObjectId(obj_category_id);
                user_repo.update_user_category(obj_user_id, &bs, UpdateType::Remove).await?;
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

    pub async fn get_categories(&self, user_id: &String) -> Result<ArrayResponse<CategoryArrayResponse>, Error> {
        let user_obj_id =  user_id.transform_to_obj_id().unwrap();
        let filter_options = doc!{"owner": user_obj_id};
        let projection = doc!{"owner": 0};
        let t = FindOptions::builder().projection(projection).build();
        let mut categories = self
            .collection
            .find(filter_options.to_owned(), t)
            .await
            .unwrap();
        let mut results: Vec<CategoryArrayResponse> = Vec::new();
        while let Some(res) = categories.next().await {
            match res {
                Ok(doc) => {
                    let response_item = CategoryArrayResponse {
                        id: doc.id,
                        category_name: doc.category_name
                    };
                    results.push(response_item)
                },
                _ => return Err(Error::DeserializationError { message: "Deserilaztion error".to_string() })
            }
        }
        let res = ArrayResponse {
            data: results
        };
        Ok(res)
    }

    pub async fn create_category(&self, category: Category, user_id: &String, user_repo: &UserRepo) -> Result<InsertOneResult, Error> {
        let owner_id = user_id.transform_to_obj_id().unwrap();
        let is_category_exists = self.get_category_by_name(&category.category_name, owner_id).await;
        match is_category_exists {
            Ok(_) => return Err(Error::DeserializationError { message: "Category with given name already exists".to_string() }),
            _ => ()
        }
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
