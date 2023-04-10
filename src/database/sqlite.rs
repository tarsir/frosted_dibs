use std::{collections::HashMap, str::FromStr};

use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Statement};

#[derive(Clone, Debug)]
pub struct TableInfoRow {
    pub id: u32,
    pub name: String,
    pub col_type: ColumnTypes,
    pub nullable: bool,
    pub default_value: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub enum ColumnTypes {
    Text,
    Integer,
    Real,
    Blob,
    #[default]
    Null,
}

impl FromStr for ColumnTypes {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "TEXT" => ColumnTypes::Text,
            "REAL" => ColumnTypes::Real,
            "INTEGER" => ColumnTypes::Integer,
            "BLOB" => ColumnTypes::Blob,
            _ => ColumnTypes::Null,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct TableDataWithColumns {
    pub data: Vec<HashMap<String, Option<String>>>,
    pub columns: Vec<TableInfoRow>,
}

pub async fn get_tables(db: DatabaseConnection) -> Result<Vec<String>, DbErr> {
    let st = Statement::from_string(
        db.get_database_backend(),
        r#"
        SELECT name FROM sqlite_schema
        WHERE type = 'table'
        AND name NOT LIKE 'sqlite_%' AND name != '_sqlx_migrations'
        ORDER BY 1
        "#
        .to_string(),
    );
    let tables: Vec<String> = db
        .query_all(st)
        .await?
        .into_iter()
        .map(|r| {
            let n: String = r
                .try_get_by(0)
                .unwrap_or_else(|_| "Application Confused".to_string());
            n
        })
        .collect();
    Ok(tables)
}

pub async fn get_table_entries<T: AsRef<str>>(
    db: DatabaseConnection,
    table: T,
) -> Result<TableDataWithColumns, DbErr> {
    let columns_st = Statement::from_string(
        db.get_database_backend(),
        format!("pragma table_info('{}')", table.as_ref()),
    );
    let columns: Vec<TableInfoRow> = db
        .query_all(columns_st)
        .await?
        .into_iter()
        .map(|r| {
            let r: (u32, String, String, bool, Option<String>) = r.try_get_many_by_index().unwrap();
            TableInfoRow {
                id: r.0,
                name: r.1,
                col_type: ColumnTypes::from_str(&r.2).unwrap_or_default(),
                nullable: r.3,
                default_value: r.4,
            }
        })
        .collect();

    let table_data_st = Statement::from_string(
        db.get_database_backend(),
        format!(r#"SELECT * FROM {} LIMIT 100"#, table.as_ref()),
    );
    let rows: Vec<HashMap<String, Option<String>>> = db
        .query_all(table_data_st)
        .await?
        .into_iter()
        .map(|r| {
            let mut row: HashMap<String, Option<String>> = HashMap::new();
            for c in &columns {
                let val: Option<String> = match c.col_type {
                    ColumnTypes::Text => {
                        Some(r.try_get_by_index(c.id as usize).unwrap_or_default())
                    }
                    ColumnTypes::Integer => {
                        if let Ok(v) = r.try_get_by_index::<i32>(c.id as usize) {
                            Some(v.to_string())
                        } else {
                            None
                        }
                    }
                    ColumnTypes::Real => {
                        if let Ok(v) = r.try_get_by_index::<f32>(c.id as usize) {
                            Some(v.to_string())
                        } else {
                            None
                        }
                    }
                    ColumnTypes::Blob => {
                        Some(r.try_get_by_index(c.id as usize).unwrap_or_default())
                    }
                    ColumnTypes::Null => None,
                };
                row.insert(String::from(&c.name), val);
            }
            row
        })
        .collect();

    Ok(TableDataWithColumns {
        columns,
        data: rows,
    })
}
