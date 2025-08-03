use crate::prelude::*;

impl crate::PermissionRepository {
    pub async fn sync<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(new: Vec<String>, client: C) -> Result<()> {
        use sqlx::query;
        
        let mut client = client
            .acquire()
            .await?;
        
        let _ = query("SET CONSTRAINTS ALL DEFERRED;").execute(&mut *client).await?;

        let old = Self::list(&mut *client)
            .await?
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>();
        
        let changes = crate::utils::detect_changes_in_vecs(old, new);

        for permission in changes.delete {
            let _ = Self::delete(&permission, &mut *client)
                .await?;
        }

        for permission in changes.create {
            let _ = Self::insert(&permission, &mut *client)
                .await?;
        }
        
        return Ok(());
    }
}
