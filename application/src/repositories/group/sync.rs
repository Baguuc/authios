use crate::prelude::*;

impl crate::GroupRepository {
    pub async fn sync<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(new: Vec<authios_domain::Group>, client: C) -> Result<()> {
        use sqlx::query;
        use crate::PermissionRepository;
        
        let mut client = client
            .acquire()
            .await?;
        
        let _ = query("SET CONSTRAINTS ALL DEFERRED;").execute(&mut *client).await;
        
        let old = Self::list(&mut *client)
            .await?;
        
        let changes = crate::utils::detect_changes_in_vecs(old, new);
        
        for group in changes.delete {
            let _ = Self::delete(&group.name, &mut *client)
                .await?;
        }

        for group in changes.create {
            let _ = Self::insert(&group.name, &mut *client)
                .await?;

            for permission in group.permissions {
                 let _ = PermissionRepository::grant(&permission, &group.name, &mut *client).await?;
            }
        }
        
        return Ok(());
    }
}
