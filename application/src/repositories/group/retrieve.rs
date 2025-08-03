use crate::prelude::*;

impl crate::GroupRepository {
    pub async fn retrieve<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<authios_domain::Group> {
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
}
