
use clean_architecture_rust::infrastructure;

pub fn main() {

	let config = infrastructure::config::AppConfig::new_config();
	let _ = infrastructure::db::new_database(&config);

	infrastructure::routing::run();
}
