use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub category_name: String,
    pub owner: Option<ObjectId>,
    pub expenses: Option<Vec<ObjectId>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryArrayResponse {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub category_name: String,
}