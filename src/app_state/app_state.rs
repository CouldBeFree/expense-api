use crate::UserRepo;

#[derive(Clone, Debug)]
pub struct AppState {
    pub user_repo: UserRepo
}