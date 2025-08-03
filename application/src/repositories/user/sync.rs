use crate::prelude::*;

impl crate::UserRepository {
    pub async fn sync<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(new: Vec<authios_domain::User>, client: C) -> Result<()> {
        use sqlx::query;
        use crate::GroupRepository;
        
        let mut client = client
            .acquire()
            .await?;

        let _ = query("SET CONSTRAINTS ALL DEFERRED;").execute(&mut *client).await;
        
        let old = Self::list(&mut *client)
            .await?;

        let changes = crate::utils::detect_changes_in_vecs(old, new);
        
        for user in changes.delete {
            let _ = Self::delete(&user.login, &mut *client)
                .await?;
        }

        for user in changes.create {
            let _ = Self::insert(&user.login, &user.pwd, &mut *client)
                .await?;

            for group in user.groups {
                 let _ = GroupRepository::grant(&group, &user.login, &mut *client)
                     .await?;
            }
        }
        
        return Ok(());
    }
}
