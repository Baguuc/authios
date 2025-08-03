use crate::prelude::*;

impl crate::GroupRepository {
    pub async fn sync(new: Vec<authios_domain::Group>, client: &sqlx::postgres::PgPool) -> Result<()> {
        use sqlx::query;
        use crate::PermissionRepository;

        let mut tx = client.begin().await?;
        
        let _ = query("SET CONSTRAINTS ALL DEFERRED;").execute(&mut *tx).await;
        
        let old = Self::list(&mut *tx)
            .await?;
        
        let changes = crate::utils::detect_changes_in_vecs(old, new);
        
        for group in changes.delete {
            let _ = Self::delete(&group.name, &mut *tx)
                .await?;
        }

        for group in changes.create {
            let _ = Self::insert(&group.name, &mut *tx)
                .await?;

            for permission in group.permissions {
                 let _ = PermissionRepository::grant(&permission, &group.name, &mut *tx).await?;
            }
        }
        
        let _ = tx.commit().await?;
        
        return Ok(());
    }
}
