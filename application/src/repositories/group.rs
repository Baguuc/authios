use crate::prelude::*;

pub struct GroupRepository;

impl GroupRepository {
    pub async fn insert<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO groups (name) VALUES ($1);";
        let result = query(sql).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn retrieve<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<authin_domain::Group> {
        use sqlx::query_as;

        let sql = "SELECT 
          g.name,
          ARRAY_REMOVE(ARRAY_AGG(gp.permission_name), NULL) AS permissions
        FROM 
          groups g
        LEFT JOIN
          group_permissions gp
          ON
          gp.group_name = g.name
        WHERE
          name = :group_name
        GROUP BY
          g.name
        ;";
        let result = query_as(sql).bind(name).fetch_one(client).await;

        match result {
            Ok(data) => return Ok(data),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn list<'c, C: sqlx::postgres::PgExecutor<'c>>(client: C) -> Result<Vec<authin_domain::Group>> {
        use sqlx::query_as;

        let sql = "SELECT 
          g.name,
          ARRAY_REMOVE(ARRAY_AGG(gp.permission_name), NULL) AS permissions
        FROM 
          groups g
        LEFT JOIN 
          group_permissions gp 
          ON 
          gp.group_name = g.name
        GROUP BY
          g.name
        ;";
        let result = query_as(sql).fetch_all(client).await;

        match result {
            Ok(data) => return Ok(data),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn delete<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM groups WHERE name = $1;";
        let result = query(sql).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn grant<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, user_login: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO user_groups (user_login, group_name) VALUES ($1, $2);";
        let result = query(sql).bind(user_login).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn revoke<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, user_login: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM user_groups WHERE user_login = $1 AND group_name = $2;";
        let result = query(sql).bind(user_login).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }

    pub async fn sync(new: Vec<authin_domain::Group>, client: &sqlx::postgres::PgPool) -> Result<()> {
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
