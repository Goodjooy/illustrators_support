use crate::database::Database;

pub mod r_result;
pub mod users;
pub mod illustrator;
pub mod admin;

#[rocket::async_trait]
pub trait SelectBy<T>{
   async fn select_by(self,db:&Database)->Result<Option<T>,sea_orm::DbErr>; 
}


#[rocket::async_trait]
pub trait TryIntoWithDatabase<T> {
    type Error;
    async fn try_into(self,db:&Database)->Result<T,Self::Error>;
}