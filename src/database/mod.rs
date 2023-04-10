use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr};

use self::sqlite::TableDataWithColumns;

pub mod sqlite;

#[derive(Default, Debug, Clone)]
pub enum SqlFlavor {
    #[default]
    Sqlite,
    Postgres,
    MySQL,
}

pub async fn get_tables(db: DatabaseConnection) -> Result<Vec<String>, DbErr> {
    match db.get_database_backend() {
        sea_orm::DatabaseBackend::Sqlite => Ok(sqlite::get_tables(db).await?),
        sea_orm::DatabaseBackend::Postgres => Ok(vec![]),
        sea_orm::DatabaseBackend::MySql => Ok(vec![]),
    }
}

pub async fn get_table_entries<T>(
    db: DatabaseConnection,
    table: T,
) -> Result<TableDataWithColumns, DbErr>
where
    T: AsRef<str>,
{
    match db.get_database_backend() {
        sea_orm::DatabaseBackend::Sqlite => Ok(sqlite::get_table_entries(db, table).await?),
        sea_orm::DatabaseBackend::Postgres => Ok(TableDataWithColumns::default()),
        sea_orm::DatabaseBackend::MySql => Ok(TableDataWithColumns::default()),
    }
}
