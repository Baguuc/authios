use crate::repositories::PermissionsRepository;

impl PermissionsRepository {
    /// # PermissionsRepository::retrieve
    ///
    /// Retrieve a permission identified by name from the database
    ///
    /// ### Arguments:
    /// + params: [crate::params::repository::permissions::PermissionRetrieveParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    pub async fn retrieve<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(
        params: crate::params::repository::PermissionRetrieveParams,
        client: C
    ) -> Result<crate::models::Permission, sqlx::Error> {
        use sqlx::query_as;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "SELECT * FROM permissions WHERE name = $1;";
        let query = query_as(sql).bind(&params.name);

        let row = query.fetch_one(&mut *client).await?;

        return Ok(row);
    }
}
