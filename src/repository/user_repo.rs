extern crate dotenv;

use std::vec;

use futures::TryStreamExt;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, UpdateResult, DeleteResult },
    Collection, Database
};

use crate::models::user_model::User;

#[derive(Clone, Debug)]
pub struct UserRepo {
    pub collection: Collection<User>
}

impl UserRepo {
    pub async fn init(db: &Database) -> Self {
        let collection: Collection<User> = db.collection("user");
        UserRepo { collection }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_user = User {
            id: None,
            name: new_user.name.to_owned(),
            email: new_user.email.to_owned(),
            password: new_user.password.to_owned(),
            expenses: Some(vec![]),
            incomes: Some(vec![])
        };
        let user = self
            .collection
            .insert_one(new_user, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
}
