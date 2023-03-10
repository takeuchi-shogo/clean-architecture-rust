#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
// #[macro_use] extern crate diesel;
// #[macro_use]extern crate rocket_contrib;
// #[macro_use]extern crate serde;

pub mod domain;
pub mod infrastructure;
pub mod interface;
pub mod usecase;
