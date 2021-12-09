use std::{error::Error, io::Cursor};

use rocket::{
    http::{ContentType, Status},
    outcome::Outcome,
    response::Responder,
    Response,
};
use serde::{ser::SerializeStruct, Serialize};

pub enum RResult<T: Serialize> {
    Success(T),
    Error(Status, String),
}

impl<T: Serialize> Serialize for RResult<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut stur = serializer.serialize_struct("ResResult", 3)?;
        match self {
            RResult::Success(data) => {
                stur.serialize_field("err", &false)?;
                stur.serialize_field("emsg", "")?;
                stur.serialize_field("data", &data)?;
            }
            RResult::Error(_status, msg) => {
                stur.serialize_field("err", &true)?;
                stur.serialize_field("emsg", msg)?;
                stur.serialize_field("data", &Option::<T>::None)?;
            }
        };
        stur.end()
    }
}

impl<T: Serialize> RResult<T> {
    fn new_success(data: T) -> Self {
        Self::Success(data)
    }

    fn new_error<M: ToString>(msg: M) -> Self {
        Self::Error(Status::NotAcceptable, msg.to_string())
    }
    fn err_with_status<M: ToString>(status: Status, msg: M) -> Self {
        Self::Error(status, msg.to_string())
    }

    pub fn from_result<E: ToString>(res: Result<T, E>) -> Self {
        match res {
            Ok(data) => Self::new_success(data),
            Err(err) => Self::new_error(err),
        }
    }
    pub fn from_option<E: ToString>(res: Option<T>, info: E) -> Self {
        Self::from_result(res.ok_or(info))
    }

    pub fn ok(data: T) -> Self {
        Self::new_success(data)
    }
    pub fn err<I: ToString>(msg: I) -> Self {
        Self::new_error(msg)
    }
    pub fn status_err<I: ToString>(status: Status, msg: I) -> Self {
        Self::err_with_status(status, msg)
    }

    pub fn into_forword(self) -> Outcome<T, (Status, String), ()> {
        match self {
            Self::Error(_, _) => Outcome::Forward(()),
            Self::Success(data) => Outcome::Success(data),
        }
    }
    pub fn change_status(self, status: Status) -> Self {
        match self {
            RResult::Error(_, m) => Self::Error(status, m),
            s => s,
        }
    }
}

impl<T: Serialize, E: Error> From<Result<T, E>> for RResult<T> {
    fn from(r: Result<T, E>) -> Self {
        Self::from_result(r)
    }
}

impl<T: Serialize> From<Option<T>> for RResult<T> {
    fn from(op: Option<T>) -> Self {
        Self::from_option(op, "None Result".to_string())
    }
}

impl<T: Serialize> Into<Result<T, String>> for RResult<T> {
    fn into(self) -> Result<T, String> {
        match self {
            RResult::Success(data) => Ok(data),
            RResult::Error(_, msg) => Err(msg),
        }
    }
}
impl<T: Serialize> Into<Option<T>> for RResult<T> {
    fn into(self) -> Option<T> {
        match self {
            RResult::Success(data) => Some(data),
            RResult::Error(_, _) => None,
        }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for RResult<T> {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let t = serde_json::to_vec(&self).or_else(|_e| Err(Status::InternalServerError))?;
        let status = match self {
            RResult::Success(_) => Status::Ok,
            RResult::Error(s, _) => s,
        };

        Response::build()
            .header(ContentType::JSON)
            .sized_body(t.len(), Cursor::new(t))
            .status(status)
            .ok()
    }
}
#[macro_export]
macro_rules! to_rresult {
    (op, $x:expr, $s:expr) => {
        match $x {
            Some(d) => d,
            None => return crate::data_containers::r_result::RResult::err($s),
        }
    };

    (op, $x:expr,  $s:expr) => {
        match $x {
            Some(d) => d,
            None => return crate::data_containers::r_result::RResult::status_err($sta, $s),
        }
    };

    (op_rev, $x:expr, $s:expr) => {
        match $x {
            Some(_) => returncrate::data_containers::r_result::RResult::err($s),
            None => (),
        }
    };

    (rs, $x:expr) => {
        match $x {
            Ok(d) => d,
            Err(err) => return crate::data_containers::r_result::RResult::err(err),
        }
    };

    (rs, $x:expr, $sta:expr) => {
        match $x {
            Ok(d) => d,
            Err(err) => return crate::data_containers::r_result::RResult::status_err($sta, err),
        }
    };

    (rs, $x:expr, $info:expr) => {
        match $x {
            Ok(d) => d,
            Err(err) => {
                return crate::data_containers::r_result::RResult::err(format!(
                    "{} => {}",
                    $info, err
                ))
            }
        }
    };

    (rs, $x:expr, $sta:expr, $info:expr) => {
        match $x {
            Ok(d) => d,
            Err(err) => {
                return crate::data_containers::r_result::RResult::status_err(
                    $sta,
                    format!("{} => {}", $info, err),
                )
            }
        }
    };
}
