use crate::utils::data_structs::r_result::RResult;
use rocket::{http::Status, Request};



pub mod config;
pub mod multpart;
pub mod data_structs;

pub mod mid_wares;

pub struct Assert<const COND: bool>;

trait IsTrue {}

impl IsTrue for Assert<true> {}

#[rocket::options("/<_..>")]
pub fn cors_handle() -> Status {
    Status::Ok
}

#[catch(default)]
pub fn catch(status:Status,_req:&Request<'_>)->RResult<()>{
    RResult::status_err(status, format!("Unhandled repsond error[{}]",status))
}