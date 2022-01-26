use std::ops::Deref;

use chrono::NaiveDateTime;
use sea_orm::{sea_query::Query, ColumnTrait, Condition};

use crate::entity::update_record;

pub struct TableName(pub &'static str);

impl Deref for TableName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TableName {
    pub const USERS: Self = Self("users");
    pub const FILE_STORES: Self = Self("file_stores");
    pub const ILLUSTRATORS: Self = Self("illustrators");
    pub const ILLUSTRATOR_WANTS: Self = Self("illustrator_wants");
    pub const ILLUSTRATOR_ACTS: Self = Self("illustrator_acts");
    pub const ADMINS: Self = Self("admins");
    pub const INVITES: Self = Self("invites");
    pub const UPDATE_RECORD: Self = Self("update_record");
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
            let mut condition = Condition::all().add(update_record::Column::ChangeTime.gt(record));

            // 如果不指定table name 就是全部更新历史
            if let Some(table_name) = table_name {
                condition = condition.add(update_record::Column::TableName.eq(table_name.deref()));
            }
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
