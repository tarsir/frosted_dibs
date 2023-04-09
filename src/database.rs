use std::collections::HashMap;
use std::fmt::format;
use sqlx::{FromRow, Row, Column, Decode, Sqlite};
use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::error::Error;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct TableInfoRow {
    pub id: u32,
    pub name: String,
    pub col_type: String,
    pub nullable: bool,
    pub default_value: String,
}

pub async fn get_tables(pool: &SqlitePool) -> Result<Vec<String>, Error> {
    let tables = sqlx::query!(
        r#"
        SELECT name FROM sqlite_schema
        WHERE type = 'table'
        AND name NOT LIKE 'sqlite_%' AND name != '_sqlx_migrations'
        ORDER BY 1
        "#
    )
    .map(|r| r.name.unwrap_or_default())
    .fetch_all(pool)
    .await?;
        
    Ok(tables)
}

pub async fn get_table_entries<T: AsRef<str>>(pool: &SqlitePool, table: T) -> Result<(Vec<HashMap<String, Option<String>>>, Vec<TableInfoRow>), Error> {
    let columns = sqlx::query(
        format!("pragma table_info('{}')", table.as_ref()).as_ref()
    )
        .fetch_all(pool)
        .await?;

    let columns: Vec<TableInfoRow> = columns
        .into_iter()
        .map(|c| {
            TableInfoRow {
                id: c.get(0),
                name: c.get(1),
                col_type: c.get(2),
                nullable: c.get(3),
                default_value: c.get(4),
            }
        })
        .collect();

    let tables = sqlx::query(
        format!("SELECT * from {}", table.as_ref()).as_ref()
    )
        .fetch_all(pool)
        .await?;


    let mut rows: Vec<_> = Vec::new();

    for r in &tables {
        let mut row: HashMap<String, Option<String>> = HashMap::new();
        for c in &columns {
            let val = r.try_get_raw(c.id as usize)?;
            let val: Option<String> = Decode::<Sqlite>::decode(val).map_err(sqlx::Error::Decode)?;
            row.insert(String::from(&c.name), val);
        }
        rows.push(row);
    }
        
    Ok((rows, columns))
}
