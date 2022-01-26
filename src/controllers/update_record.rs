use crate::{
    data_containers::{
        update_record::{LastUpdate, RecordUnit},
        SelectBy,
    },
    to_rresult, Database,
};

use rocket::{http::Status, State};

use crate::utils::data_structs::{header_info::HeaderInfo, r_result::RResult};

crate::generate_controller!(UpdateRecordController, "/update", check_uptated);

#[post("/check")]
async fn check_uptated(update: HeaderInfo<'_, LastUpdate>, db: &State<Database>) -> RResult<usize> {
    if let Some(time) = update.try_into().ok() {
        let rec = RecordUnit(time);

        if let Some(count) = to_rresult!(rs, rec.select_by(db).await) {
            RResult::ok(count)
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
