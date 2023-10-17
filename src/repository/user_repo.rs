extern crate dotenv;

use std::{vec, f64::consts::E};

use bcrypt::{DEFAULT_COST, hash};
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, Bson},
    // results::{ InsertOneResult, UpdateResult, DeleteResult },
    Collection, Database
};

use crate::models::user_model::{User, UserResponse, UserLogin};

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

    async fn get_user_by_email(&self, user_email: &String) -> Result<User, Error> {
        let filter_options = doc! {"email": user_email};
        let user = self
            .collection
            .find_one(filter_options, None)
            .await
            .ok()
            .unwrap();
        match user {
            Some(data) => Ok(data),
            None => Err(Error::DeserializationError { message: "No user found".to_string() })
        }
    }

    pub async fn create_user(&self, new_user: User) -> Result<UserResponse, Error> {
        let hashed_password: String = hash(new_user.password.as_str(), DEFAULT_COST).unwrap();
        let email_result = self.get_user_by_email(&new_user.email).await;
        match email_result {
            Ok(_) => Err(Error::DeserializationError { message: "User with this email already registered".to_string() }),
            Err(_) => {
                let new_user = User {
                    id: None,
                    name: new_user.name.to_owned(),
                    email: new_user.email.to_owned(),
                    password: hashed_password,
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
    }

    pub async fn login_user(&self, user: UserLogin) -> Result<User, Error> {
        let user_result = self.get_user_by_email(&user.email).await;
        match user_result {
            Ok(data) => {
                let is_password_valid = data.verify_password(&user.password);
                if !is_password_valid {
                    return Err(Error::DeserializationError { message: "Email or password invalid".to_string() })
                } else {
                    return Ok(data);
                }
            },
            Err(_) => return Err(Error::DeserializationError { message: "Email or password invalid".to_string() })
        }
    }
}
