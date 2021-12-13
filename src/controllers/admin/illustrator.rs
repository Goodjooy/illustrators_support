use rocket::{http::Status, serde::json::Json, State};
use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::{
    controllers::admin::illustrator,
    data_containers::{admin::Admin, illustrator::IllustratorOp, r_result::RResult},
    database::Database,
    entity::illustrators,
    to_rresult,
};

#[put("/illustrator/<iid>", data = "<updata_ill>")]
pub async fn edit(
    _auth: Admin,
    iid: i64,
    updata_ill: Json<IllustratorOp>,
    db: &State<Database>,
) -> RResult<()> {
    let inner_upper = updata_ill.into_inner();

    let src = crate::to_rresult!(
        op,
        crate::to_rresult!(
            rs,
            illustrators::Entity::find_by_id(iid).one(db.unwarp()).await
        ),
        Status::NotFound,
        "Target illustrator Not Found"
    );

    let update = inner_upper.mix_with_model(src);

    let _res = to_rresult!(rs, update.update(db.unwarp()).await);

    RResult::ok(())
}
