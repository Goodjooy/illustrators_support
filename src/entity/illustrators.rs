//! SeaORM Entity. Generated by sea-orm-codegen 0.4.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "illustrators")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub id: i64,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(unique)]
    pub home: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::illustrator_acts::Entity")]
    IllustratorActs,
    #[sea_orm(has_many = "super::illustrator_wants::Entity")]
    IllustratorWants,
}

impl Related<super::illustrator_acts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IllustratorActs.def()
    }
}

impl Related<super::illustrator_wants::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IllustratorWants.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
