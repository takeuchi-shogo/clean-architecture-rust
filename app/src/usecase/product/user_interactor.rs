
// use chrono::Local;
use std::time::{ SystemTime, UNIX_EPOCH };

use crate::domain::users::Users;
pub struct UserInteractor {

}


impl UserInteractor {

	pub fn get_user(&self) -> Users {

		let now = SystemTime::now();
		let unix_time = now.duration_since(UNIX_EPOCH).expect("back to future");
		let user = Users {
			id: 100,
			screen_name: String::from("testtest"),
			display_name: String::from("test taro"),
			email: String::from("test@example.com"),
			password: String::from("okokok"),
			created_at: unix_time.as_secs(),
			updated_at: unix_time.as_secs(),
			deleted_at: 0,
		};

		return user
	}
}
