use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, Bson},
    results::UpdateResult, Database
};

use crate::models::user_model::{User, UserResponse, UserLogin};
use crate::utils::UpdateType;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn init(db: &Database) -> Self;
    async fn update_user_income(&self, user_id: ObjectId, post_id: &Bson, update_type: UpdateType) -> Result<UpdateResult, Error>;
    async fn find_user_by_id(&self, user_id: &Bson) -> Result<UserResponse, Error>;
    async fn get_user_by_email(&self, user_email: &String) -> Result<User, Error>;
    async fn create_user(&self, new_user: User) -> Result<UserResponse, Error>;
    async fn login_user(&self, user: UserLogin) -> Result<User, Error>;
}