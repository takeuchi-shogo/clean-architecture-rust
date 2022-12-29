
use crate::{usecase::product::user_interactor::UserInteractor, domain::users::Users};

pub struct UsersController {
	pub interactor: UserInteractor,
}


pub fn new_user_controller() -> UsersController {
	UsersController { interactor: UserInteractor {  } }
}


impl UsersController {

	pub fn get_user(&self) -> Users {

		return self.interactor.get_user();
	}
}
