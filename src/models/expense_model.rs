use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Expense {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub expense_name: String,
    pub date: String,
    pub owner: Option<ObjectId>,
    pub category_id: ObjectId,
    pub amount: usize
}