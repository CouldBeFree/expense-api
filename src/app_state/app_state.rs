use crate::UserRepo;
use crate::IncomeRepo;

#[derive(Clone, Debug)]
pub struct AppState {
    pub user_repo: UserRepo,
    pub income_repo: IncomeRepo
}