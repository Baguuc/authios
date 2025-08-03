use crate::prelude::*;

impl crate::UserRepository {
    pub async fn check_permission<'c, C: sqlx::postgres::PgExecutor<'c>>(login: &String, permission_name: &String, client: C) -> Result<bool> { 
        use sqlx::query;

        let sql = "    
        SELECT 
          p.name
        FROM 
          permissions p 
        INNER JOIN 
          group_permissions gp 
          ON 
          p.name = gp.permission_name
        INNER JOIN 
          groups g
          ON 
          g.name = gp.group_name
        INNER JOIN 
          user_groups ug
          ON
          g.name = ug.group_name
        INNER JOIN
          users u
          ON
          u.login = ug.user_login
        WHERE 
          u.login = $1 
          AND
          p.name = $2
        ;
        ";
        let result = query(sql).bind(login).bind(permission_name).execute(client).await;
        
        match result {
            Ok(info) => return Ok(info.rows_affected() > 0),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
