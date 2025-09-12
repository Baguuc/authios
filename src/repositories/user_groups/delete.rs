use crate::repositories::UserGroupsRepository;

impl UserGroupsRepository {
    /// # UserGroupsRepository::delete
    ///
    /// Revoke user a group, deleting it from user_groups table in the database
    ///
    /// ### Arguments:
    /// + params: [crate::params::repository::user_groups::UserGroupDeleteParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserGroupDeleteParams,
        client: A
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "DELETE FROM user_groups WHERE user_login = $1 AND group_name = $2;";
        let query = query(sql).bind(&params.user_login).bind(&params.group_name);

        let result = query.execute(&mut *client).await?;

        return Ok(result);
    }
}
