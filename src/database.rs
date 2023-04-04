use sqlx::sqlite::SqlitePool;
use sqlx::error::Error;

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
