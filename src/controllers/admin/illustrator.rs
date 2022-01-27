use rocket::{http::Status, serde::json::Json, State};
use sea_orm::{ActiveModelTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter};

use crate::{
    data_containers::{admin::Admin, illustrator::IllustratorOp},
    database::Database,
    entity::illustrators,
    to_rresult,
    utils::{config::Config, data_structs::r_result::RResult},
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

#[delete("/illrstrator/<iid>")]
pub async fn remove(
    _auth: Admin,
    iid: i64,
    db: &State<Database>,
    config: &State<Config>,
) -> RResult<()> {
    let file_root = config.consts.save_dir.clone();
    // remove referent files;
    // get all ref to files
    let _res = to_rresult!(
        rs,
        db.unwarp()
            .transaction::<_, _, sea_orm::DbErr>(|ctx| {
                use sea_orm::ColumnTrait;
                Box::pin(async move {
                    let file_root = std::path::Path::new(&file_root);
                    // delete wants
                    use crate::entity::illustrator_wants;
                    illustrator_wants::Entity::delete_many()
                        .filter(illustrator_wants::Column::Iid.eq(iid))
                        .exec(ctx)
                        .await?;

                    // find out all relate files
                    use crate::entity::file_stores;
                    use crate::entity::illustrator_acts;
                    let files = illustrator_acts::Entity::find()
                        .filter(illustrator_acts::Column::Iid.eq(iid))
                        .all(ctx)
                        .await?
                        .into_iter()
                        .map(|m| m.fid)
                        .map(|m| file_stores::Column::Id.eq(m))
                        .collect::<Vec<_>>();
                    if files.len() > 0 {
                        let mut condition = Condition::any();
                        for f in files {
                            condition = condition.add(f);
                        }

                        // read all files
                        let all_files = file_stores::Entity::find()
                            .filter(condition.clone())
                            .all(ctx)
                            .await?;
                        //remove file
                        for f in all_files {
                            let filename = f.file;
                            let file_path = file_root.join(&filename);
                            // remove file
                            let _ = rocket::tokio::fs::remove_file(file_path).await;
                        }
                        // remove relate
                        illustrator_acts::Entity::delete_many()
                            .filter(illustrator_acts::Column::Iid.eq(iid))
                            .exec(ctx)
                            .await?;

                        // remove file handle in db
                        file_stores::Entity::delete_many()
                            .filter(condition)
                            .exec(ctx)
                            .await?;
                    }

                    Ok(())
                })
            })
            .await
    );

    Some(()).into()
}
