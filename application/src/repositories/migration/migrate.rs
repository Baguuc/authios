use crate::prelude::*;

impl crate::MigrationRepository {
    pub async fn migrate(client: &sqlx::postgres::PgPool) -> Result<()> {
        use sqlx::query;

        let mut tx = client.begin().await?;

        for sql in crate::repositories::migration::MIGRATIONS {
            let _ = query(sql).execute(&mut *tx).await?;
        }

        let _ = tx.commit().await?;
        
        return Ok(());
    }
}
