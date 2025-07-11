use crate::prelude::*;

pub struct PermissionRepository;

impl PermissionRepository {
    pub async fn insert<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO permissions (name) VALUES ($1);";
        let result = query(sql).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn retrieve<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<authios_domain::Permission> {
        use sqlx::query_as;

        let sql = "SELECT * FROM permissions WHERE name = $1;";
        let result = query_as(sql).bind(name).fetch_one(client).await;

        match result {
            Ok(data) => return Ok(data),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn list<'c, C: sqlx::postgres::PgExecutor<'c>>(client: C) -> Result<Vec<authios_domain::Permission>> {
        use sqlx::query_as;

        let sql = "SELECT * FROM permissions;";
        let result = query_as(sql).fetch_all(client).await;

        match result {
            Ok(data) => return Ok(data),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn delete<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM permissions WHERE name = $1;";
        let result = query(sql).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn grant<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, group_name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO group_permissions (group_name, permission_name) VALUES ($1, $2);";
        let result = query(sql).bind(group_name).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn revoke<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, group_name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM group_permissions WHERE group_name = $1 AND permission_name = $2;";
        let result = query(sql).bind(group_name).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }

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
