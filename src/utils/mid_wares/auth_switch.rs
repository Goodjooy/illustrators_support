use rocket::{
    fairing::{Fairing, Kind},
    http::{Cookie, Header, hyper},
    Data, Request, Response,
};

const HEAD_NAME: &str = "Authenticated";
pub struct AuthSwitch {
    patten: regex::Regex,
}

impl AuthSwitch {
    pub fn new() -> AuthSwitch {
        AuthSwitch {
            patten: regex::RegexBuilder::new(r#"^(.+?):=(.+?)$"#)
                .case_insensitive(true)
                .build()
                .expect("Woring Regex Expression"),
        }
    }
}

#[async_trait]
impl Fairing for AuthSwitch {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Auth Switch",
            kind: Kind::Response | Kind::Request,
        }
    }
    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        let auth_iter = req.headers().get(HEAD_NAME);
        for auth in auth_iter {
            if self.patten.is_match(auth) {
                let cap = self.patten.captures(auth).unwrap();
                let name = cap.get(1).unwrap().as_str().to_owned();
                let value = cap.get(2).unwrap().as_str().to_owned();
                let cookie = Cookie::new(name, value);
                req.cookies().add(cookie);
            }
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        let headers = res
            .cookies()
            .map(|c| Header::new(HEAD_NAME, format!("{}:={}", c.name(), c.value())))
            .collect::<Vec<_>>();

        for h in headers {
            res.adjoin_header(h);
        }

        res.adjoin_raw_header(hyper::header::ACCESS_CONTROL_EXPOSE_HEADERS.as_str(),HEAD_NAME);
    }
}
