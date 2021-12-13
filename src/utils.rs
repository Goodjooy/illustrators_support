use rocket::{http::Status, Request};

use crate::data_containers::r_result::RResult;

use self::range_limit::RangeLimit;

pub mod config;
pub mod crypto_string;
pub mod lifetime_hashmap;
pub mod measureable;
pub mod multpart;
pub mod range_limit;

pub mod mid_wares;

pub struct Assert<const COND: bool>;

trait IsTrue {}

impl IsTrue for Assert<true> {}

pub type RangeLimitString<const L: usize, const H: usize> = RangeLimit<String, L, H>;
pub type MaxLimitString<const H: usize> = RangeLimitString< 0, H>;

pub type RangeLimitVec<T, const L: usize, const H: usize> = RangeLimit<Vec<T>, L, H>;

#[rocket::options("/<_..>")]
pub fn cors_handle() -> Status {
    Status::Ok
}

#[catch(default)]
pub fn catch(status:Status,_req:&Request<'_>)->RResult<()>{
    RResult::status_err(status, format!("Unhandled repsond error[{}]",status))
}