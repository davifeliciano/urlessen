use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("urlessen")]
pub struct Db(sqlx::PgPool);
