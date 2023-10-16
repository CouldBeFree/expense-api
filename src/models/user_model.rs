extern crate bcrypt;

use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use email_address::EmailAddress;
use bcrypt::verify;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub expenses: Option<Vec<ObjectId>>,
    pub incomes: Option<Vec<ObjectId>>,
}

impl User {
    pub fn is_valid_email(&self) -> bool {
        EmailAddress::is_valid(&self.email)
    }

    pub fn verify_password(&self, password: String) -> bool {
        verify(password.as_str(), &self.password).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
}
