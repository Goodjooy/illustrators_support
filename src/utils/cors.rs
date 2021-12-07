use rocket::{
    fairing::{Fairing, Kind},
    http::{
        hyper::header::{
            ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
            ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
        },
        Header,
    },
};

pub struct Cors;

#[async_trait]
impl Fairing for Cors {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Cors",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        res.adjoin_header(Header::new(ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), "*"));
        res.adjoin_header(Header::new(
            ACCESS_CONTROL_ALLOW_METHODS.as_str(),
            "POST,GET,PATCH,DELETE,OPTIONS,PUT",
        ));
        res.adjoin_header(Header::new(
            ACCESS_CONTROL_ALLOW_CREDENTIALS.as_str(),
            "true",
        ));
        res.adjoin_header(Header::new(ACCESS_CONTROL_ALLOW_HEADERS.as_str(), "*"));
    }
}
