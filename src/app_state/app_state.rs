use crate::UserRepo;
use crate::IncomeRepo;
use crate::CategoryRepo;

#[derive(Clone, Debug)]
pub struct AppState {
    pub user_repo: UserRepo,
    pub income_repo: IncomeRepo,
    pub category_repo: CategoryRepo
}