use rocket::{
    fairing::{Fairing, Kind},
    http::{
        hyper::{
            self,
            header::{
                ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
                ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, ORIGIN,
            },
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

    async fn on_response<'r>(&self, req: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        let header = req.headers();
        if let Some(s) = header.get_one(ORIGIN.as_str()) {
            res.adjoin_header(Header::new(ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), s));
        }

        if let Some(s) = header.get_one(hyper::header::ACCESS_CONTROL_REQUEST_METHOD.as_str()) {
            res.adjoin_header(Header::new(ACCESS_CONTROL_ALLOW_METHODS.as_str(), s));
        }
        res.adjoin_header(Header::new(
            ACCESS_CONTROL_ALLOW_CREDENTIALS.as_str(),
            "true",
        ));

        for h in header.get(hyper::header::ACCESS_CONTROL_REQUEST_HEADERS.as_str()) {
            res.adjoin_header(Header::new(ACCESS_CONTROL_ALLOW_HEADERS.as_str(), h));
        }

    }
}
