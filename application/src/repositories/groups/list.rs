impl crate::GroupsRepository {
    /// # GroupsRepository::list
    ///
    /// list all groups in the database
    ///
    pub async fn list<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(client: A) -> Result<Vec<authios_domain::Group>, sqlx::Error> {
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
        GROUP BY
          g.name;
        ;";
        let query = query_as(sql);

        let rows = query.fetch_all(&mut *client).await?;
        
        return Ok(rows);
    }
}
