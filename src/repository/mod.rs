pub mod user_repo;
pub mod income_repo;
pub mod category_repo;
pub mod expense_repo;

use crate::utils::{UpdateType, Pagination, ArrayResponse, QueryParams, ParseStringToObjId};
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, Bson},
    results::{ InsertOneResult, DeleteResult },
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Collection, Database
};

impl ParseStringToObjId for String {
    fn transform_to_obj_id(&self) -> Result<ObjectId, mongodb::bson::oid::Error> {
        ObjectId::parse_str(self)
    }
}