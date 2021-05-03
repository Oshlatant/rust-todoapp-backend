use std::sync::Mutex;
use random_number::random_ranged;

pub struct JsonDb {
	pub content: Mutex<String>,
}


pub fn random_id() -> i32 {
	random_ranged(1..=10000)
}