use std::path::PathBuf;

use rocket::http::Status;

use self::range_limit::RangeLimit;

pub mod config;
pub mod cors;
pub mod crypto_string;
pub mod lifetime_hashmap;
pub mod measureable;
pub mod multpart;
pub mod range_limit;
pub mod auth_switch;

pub struct Assert<const COND: bool>;

trait IsTrue {}

impl IsTrue for Assert<true> {}

pub type RangeLimitString<const L: usize, const H: usize> = RangeLimit<String, L, H>;
pub type MaxLimitString<const H: usize> = RangeLimitString< 0, H>;

pub type RangeLimitVec<T, const L: usize, const H: usize> = RangeLimit<Vec<T>, L, H>;

#[rocket::options("/<_indata..>")]
pub fn cors_handle(_indata: PathBuf) -> Status {
    Status::Ok
}

