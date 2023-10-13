extern crate dotenv;

// use futures::TryStreamExt;
// use mongodb::{
//     bson::{extjson::de::Error, oid::ObjectId, doc},
//     results::{ InsertOneResult, UpdateResult, DeleteResult },
//     Collection, Database
// };

use mongodb::{Collection, Database};

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
}
