use rocket::{
    fairing::{Fairing, Kind},
    Data, Request, Response,
};

/**
 * @Author: Your name
 * @Date:   2021-12-11 13:35:17
 * @Last Modified by:   Your name
 * @Last Modified time: 2021-12-11 18:19:15
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
        log::info!("--------------------------------------------");
        log::info!(
            "Request Entry from {} | {}",
            into_unknow_string(req.client_ip()),
            req.uri()
        );
        log::info!("Method: {}", req.method());
        log::info!("Content type: {}", into_unknow_string(req.content_type()));
        log::info!("--------------------------------------------");
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        log::info!("--------------------------------------------");
        log::info!("Request Leave for {}", into_unknow_string(req.client_ip()));
        log::info!("Status: {}", res.status());
        log::info!("Content type: {}", into_unknow_string(res.content_type()));
        log::info!("--------------------------------------------");
    }
}
