use std::ops::Deref;

use chrono::NaiveDateTime;
use sea_orm::{sea_query::Query, ColumnTrait, Condition};

use crate::entity::update_record;

pub struct TableName<'s>(pub &'s str);

impl<'s> Deref for TableName<'s> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'s, T> From<&'s T> for TableName<'s>
where
    T: sea_orm::EntityName + sea_orm::EntityTrait,
{
    fn from(en: &'s T) -> Self {
        TableName(en.table_name())
    }
}

impl<'s> TableName<'s> {
    pub fn into_op(self) -> Option<Self> {
        Some(self)
    }

    pub const USERS: Option<Self> = Some(Self("users"));
    pub const FILE_STORES: Option<Self> = Some(Self("file_stores"));
    pub const ILLUSTRATORS: Option<Self> = Some(Self("illustrators"));
    pub const ILLUSTRATOR_WANTS: Option<Self> = Some(Self("illustrator_wants"));
    pub const ILLUSTRATOR_ACTS: Option<Self> = Some(Self("illustrator_acts"));
    pub const ADMINS: Option<Self> = Some(Self("admins"));
    pub const INVITES: Option<Self> = Some(Self("invites"));
    pub const UPDATE_RECORD: Option<Self> = Some(Self("update_record"));
}

pub trait UpdateRecordBound {
    fn ext_record_bound<C: ColumnTrait>(
        self,
        column: &C,
        table_name: Option<TableName>,
        record: Option<NaiveDateTime>,
    ) -> Condition;
}

impl UpdateRecordBound for Condition {
    fn ext_record_bound<C: ColumnTrait>(
        self,
        column: &C,
        table_name: Option<TableName>,
        record: Option<NaiveDateTime>,
    ) -> Condition {
        if let Some(record) = record {
            let condition = record_condition(table_name, record);
            Condition::all().add(self).add(
                column.in_subquery(
                    Query::select()
                        .column(update_record::Column::ChangeId)
                        .from(update_record::Entity)
                        .cond_where(condition)
                        .to_owned(),
                ),
            )
        } else {
            self
        }
    }
}

pub fn record_condition(table_name: Option<TableName>, record: NaiveDateTime) -> Condition {
    let mut condition = Condition::all().add(update_record::Column::ChangeTime.gt(record));

    // ???????????????table name ????????????????????????
    if let Some(table_name) = table_name {
        condition = condition.add(update_record::Column::TableName.eq(table_name.deref()));
    }

    condition
}
