
use crate::interface::controllers::product::users_controller;

// use rocket::http::hyper::mime::Value;
use serde_json::Result;



#[get("/")]
fn index() -> &'static str {
	"hello world"
}

#[get("/users")]
fn get_user() -> Result<std::string::String> {
	let users_controller = users_controller::new_user_controller();
	let user = users_controller.get_user();
	// Json(user)
	return serde_json::to_string(&user);
// 	println!("{}", j);
// 	// Ok(());
}


pub fn run() {
	rocket::ignite()
		.mount("/api",
			routes![
				index,
				get_user,
			],
		)
		.launch();
}
