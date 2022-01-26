use crate::{ to_rresult, Database};

use rocket::{http::Status, State};

use crate::utils::data_structs::{header_info::HeaderInfo, r_result::RResult};

crate::generate_controller!(UpdateRecordController, "/update", check_uptated);

crate::header_captures!(pub LastUpdate : "Last-Update");

#[post("/check")]
async fn check_uptated(update: HeaderInfo<'_, LastUpdate>, db: &State<Database>) -> RResult<()> {
    if let Some(time) = update
        .get_one()
        .and_then(|h| chrono::NaiveDateTime::parse_from_str(h, "%Y-%m-%d %H:%M:%S").ok())
    {
        if false {
            RResult::ok(())
        } else {
            RResult::status_err(Status::NotModified, "All are the newest")
        }
    } else {
        RResult::status_err(Status::PreconditionFailed, "No update Infomation")
    }
}

#[cfg(test)]
mod test_update {

    use rocket::{http::Status, local::blocking::Client};

    use crate::{controllers::Controller, utils::data_structs::header_info::FromHeaders};

    use super::{LastUpdate, UpdateRecordController};

    fn mock_client() -> Client {
        let serve = rocket::build().mount(
            UpdateRecordController::base(),
            UpdateRecordController::routes(),
        );

        let client = Client::tracked(serve).expect("Mock client Create Failure");

        client
    }

    // static CLIENT:Arc<Client>=Arc::new(mock_client());

    #[test]
    fn test_update_without_head() {
        let clinet = mock_client();
        let req = clinet.post("/update/check");
        let res = req.dispatch();

        assert_eq!(res.status(), Status::PreconditionFailed);
    }

    #[test]
    fn test_update_with_new() {
        let clinet = mock_client();
        let mut req = clinet.post("/update/check");
        let header = rocket::http::Header::new(LastUpdate::header_name(), "2023-1-1 12:10:00");
        req.add_header(header);
        let res = req.dispatch();

        assert_eq!(res.status(), Status::Ok);
    }
}
