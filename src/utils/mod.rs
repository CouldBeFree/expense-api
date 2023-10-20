use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Error {
    pub error: String
}

pub enum UpdateType {
    Add,
    Remove
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub current: usize,
    pub count: u64,
    pub next: usize,
    pub previous: Option<usize>,
    pub pages: u64,
    pub per_page: usize,
}

#[derive(Debug, Serialize)]
pub struct ArrayResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination
}

#[derive(Debug)]
pub struct QueryParams {
    pub per_page: usize,
    pub page: usize,
}

impl QueryParams {
    pub fn new(per_page: Option<&String>, page: Option<&String>) -> Self {
        let page = match page {
            Some(p) => p.parse().unwrap(),
            None => 1
        };
        let per_page = match per_page {
            Some(p) => p.parse().unwrap(),
            None => 4
        };
        QueryParams { 
            per_page,
            page
         }
    }
}
