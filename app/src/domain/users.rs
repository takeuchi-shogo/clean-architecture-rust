
use serde::{ Serialize };

#[derive(Debug, Serialize)]
pub struct Users {
	pub id: u32,
	pub display_name: String,
	pub screen_name: String,
	pub email: String,
	pub password: String,

	pub created_at: u64,
	pub updated_at: u64,
	pub deleted_at: i64,
}
