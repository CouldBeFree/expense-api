use crate::utils::traits::user::UserRepositoryTrait;
use crate::models::user_model::User;
use bcrypt::{DEFAULT_COST, hash};
use actix_web::web::Json;
// use crate::UserRepository as Respository;
// use  crate::service::user_service::UserRepository as Repository;
// use crate::repository::user_repo::UserRepo;

pub struct UserService<T: UserRepositoryTrait> {
    repository: T
}

impl<T: UserRepositoryTrait> UserService<T> {
    pub async fn create(&self, new_user: Json<User>) -> () {
        let hashed_password: String = hash(new_user.password.as_str(), DEFAULT_COST).unwrap();
        let email_result = self.repository.get_user_by_email(&new_user.email).await;
    }
}
