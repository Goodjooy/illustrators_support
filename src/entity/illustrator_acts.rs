//! SeaORM Entity. Generated by sea-orm-codegen 0.3.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "illustrator_acts")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub id: i64,
    pub iid: i64,
    #[sea_orm(unique)]
    pub fid: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::illustrators::Entity",
        from = "Column::Iid",
        to = "super::illustrators::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Illustrators,
    #[sea_orm(
        belongs_to = "super::file_stores::Entity",
        from = "Column::Fid",
        to = "super::file_stores::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    FileStores,
}

impl Related<super::illustrators::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Illustrators.def()
    }
}

impl Related<super::file_stores::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileStores.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
