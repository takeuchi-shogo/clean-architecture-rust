
use diesel::{ self, Connection };
use diesel::mysql::MysqlConnection;

use std::thread;
use std::time::Duration;

use super::config::AppConfig;


pub struct DB {
	// pub host: String,
	// pub db_name: String,
	// pub user_name: String,
	// pub password: String,

	pub connection: MysqlConnection,
}


pub fn new_database(c: &AppConfig) -> DB {
	DB {
		// host: c.db.production.host.clone(),
		// db_name: c.db.production.db_name.clone(),
		// user_name: c.db.production.user_name.clone(),
		// password: c.db.production.password.clone(),
		connection: DB::establish_connection(
			&c.db.production.user_name,
			&c.db.production.password,
			&c.db.production.host,
			&c.db.production.db_name,
		),
	}
}

impl DB {
	pub fn establish_connection(
		user_name: &String,
		password: &String,
		host: &String,
		db_name: &String) -> MysqlConnection {
		let mut counter =0;
		loop {
			println!("{}", ".");
			thread::sleep(Duration::from_secs(1));
			if counter == 5{
				break counter;
			}
			counter += 1;
		};

		// フォーマット時にスペースを入れるとエラーが出る { xxxxx } -> {xxxxx}
		macro_rules! database_url { () => ("mysql://{user_name}:{password}@{host}:3306/{db_name}")}
		// let connect = "mysql://root:@localhost/" + self.db_name;
		let database_url = format!(
			database_url!(),
			user_name = user_name,
			password = password,
			host = host,
			db_name = db_name,
		);
		MysqlConnection::establish(&database_url)
			.unwrap_or_else(|_| panic!("Error connection to {}", &database_url))
	}
}
