pub async fn migrate<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(client: C) -> Result<(), sqlx::Error> {
    use sqlx::query;

    let mut client = client
        .acquire()
        .await?;

    for sql in crate::MIGRATIONS {
        let _ = query(sql).execute(&mut *client).await?;
    }
    
    return Ok(());
}
