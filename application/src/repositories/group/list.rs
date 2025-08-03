use crate::prelude::*;

impl crate::GroupRepository {
    pub async fn list<'c, C: sqlx::postgres::PgExecutor<'c>>(client: C) -> Result<Vec<authios_domain::Group>> {
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
}
