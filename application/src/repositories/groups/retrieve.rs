impl crate::GroupsRepository {
    /// # GroupsRepository::retrieve
    ///
    /// retrieve a group identified by name from the database
    ///
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(name: &String, client: A) -> Result<authios_domain::Group, sqlx::Error> {
        use sqlx::query_as;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "
        SELECT
          g.name,
          ARRAY_REMOVE(ARRAY_AGG(gp.permission_name), NULL) AS permissions
        FROM 
          groups g
        LEFT JOIN 
          group_permissions gp 
          ON 
          gp.group_name = g.name
        WHERE
          g.name = $1
        GROUP BY
          g.name;
        ";
        let query = query_as(sql).bind(name);

        let row = query.fetch_one(&mut *client).await?;

        return Ok(row);
    }
}
