use crate::prelude::*;

impl crate::PermissionRepository {
    pub async fn sync(new: Vec<String>, client: &sqlx::postgres::PgPool) -> Result<()> {
        use sqlx::query;

        let mut tx = client.begin().await?;
        
        let _ = query("SET CONSTRAINTS ALL DEFERRED;").execute(&mut *tx).await?;

        let old = Self::list(&mut *tx)
            .await?
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>();
        
        let changes = crate::utils::detect_changes_in_vecs(old, new);

        for permission in changes.delete {
            let _ = Self::delete(&permission, &mut *tx)
                .await?;
        }

        for permission in changes.create {
            let _ = Self::insert(&permission, &mut *tx)
                .await?;
        }

        let _ = tx.commit().await?;
        
        return Ok(());
    }
}
