use log::info;
use rocket::{
    fairing::{Fairing, Kind},
    Data, Request, Response,
};

/**
 * @Author: Your name
 * @Date:   2021-12-11 13:35:17
 * @Last Modified by:   Your name
 * @Last Modified time: 2021-12-11 21:16:22
 */

pub struct NetLogger;

const UNK: &str = "unknown";
fn into_unknow_string<T: ToString>(data: Option<T>) -> String {
    data.and_then(|d| Some(d.to_string()))
        .unwrap_or(String::from(UNK))
}

#[async_trait]
impl Fairing for NetLogger {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Net Logger",
            kind: Kind::Response | Kind::Request,
        }
    }
    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        info!(
            "--------------------------------------------\nRequest Entry from {} | {}\nMethod: {}\nContent type: {}\n--------------------------------------------"
        , into_unknow_string(req.client_ip()),
        req.uri()
        , req.method()
        , into_unknow_string(req.content_type())
        );
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        info!("--------------------------------------------\nRespond Leave for {} | {}\nStatus: {}\nContent type: {}\n--------------------------------------------"
        ,
        into_unknow_string(req.client_ip()),
        req.uri(), res.status(), into_unknow_string(res.content_type())
    );
    }
}
