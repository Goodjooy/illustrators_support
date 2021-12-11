/**
 * @Author: Your name
 * @Date:   2021-12-01 19:34:15
 * @Last Modified by:   Your name
 * @Last Modified time: 2021-12-11 17:52:19
 */
use std::path::PathBuf;

use rocket::{http::Status, Request};

use crate::data_containers::r_result::RResult;

use self::range_limit::RangeLimit;

pub mod config;
pub mod cors;
pub mod crypto_string;
pub mod lifetime_hashmap;
pub mod measureable;
pub mod multpart;
pub mod range_limit;
pub mod auth_switch;
pub mod net_logging;

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

#[catch(default)]
pub fn catch(status:Status,_req:&Request<'_>)->RResult<()>{
    RResult::status_err(status, format!("Unhandled repsond error[{}]",status))
}