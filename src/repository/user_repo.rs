extern crate dotenv;

use std::vec;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, Bson},
    results::{ InsertOneResult, UpdateResult, DeleteResult },
    Collection, Database
};

use crate::models::user_model::{User, UserResponse};

#[derive(Clone, Debug)]
pub struct UserRepo {
    pub collection: Collection<User>
}

impl UserRepo {
    pub async fn init(db: &Database) -> Self {
        let collection: Collection<User> = db.collection("user");
        UserRepo { collection }
    }

    async fn find_user_by_id(&self, user_id: &Bson) -> Result<UserResponse, Error> {
        let filter_options = doc!{"_id": user_id};
        let user = self
            .collection
            .find_one(filter_options, None)
            .await
            .unwrap();
        match user {
            Some(u) => {
                return Ok(UserResponse{name: u.name, email: u.email, id: u.id})
            },
            None => Err(Error::DeserializationError { message: "User not found".to_string() })
        }
    }

    pub async fn create_user(&self, new_user: User) -> Result<UserResponse, Error> {
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
        let oid = &user.inserted_id;
        let create_user = self.find_user_by_id(&oid).await?;
        Ok(create_user)
    }
}
