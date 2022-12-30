
// use diesel::{self, Connection};
// use diesel::mysql::MysqlConnection;

// use super::config::AppConfig;


// pub struct DB {
// 	pub host: String,
// 	pub db_name: String,
// 	pub user_name: String,
// 	pub password: String,

// 	pub connection: MysqlConnection,
// }


// impl DB {
// 	pub fn new_database(c: &AppConfig) -> DB {
// 		DB {
// 			host: c.db.production.host.clone(),
// 			db_name: c.db.production.db_name.clone(),
// 			user_name: c.db.production.user_name.clone(),
// 			password: c.db.production.password.clone(),
// 			connection: DB::establish_connection(),
// 		}
// 	}

// 	pub fn establish_connection() -> MysqlConnection {
// 		let database_url = String::from("mysql://root:@localhost/diesel_test");
// 		MysqlConnection::establish(&database_url)
// 			.unwrap_or_else(|_| panic!("Error connection to {}", &database_url))
// 	}
// }
