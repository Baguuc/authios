use crate::prelude::*;

impl crate::UserRepository {
    pub async fn sync(new: Vec<authios_domain::User>, client: &sqlx::postgres::PgPool) -> Result<()> {
        use sqlx::query;
        use crate::GroupRepository;

        let mut tx = client.begin().await?;

        let _ = query("SET CONSTRAINTS ALL DEFERRED;").execute(&mut *tx).await;
        
        let old = Self::list(&mut *tx)
            .await?;

        let changes = crate::utils::detect_changes_in_vecs(old, new);
        
        for user in changes.delete {
            let _ = Self::delete(&user.login, &mut *tx)
                .await?;
        }

        for user in changes.create {
            let _ = Self::insert(&user.login, &user.pwd, &mut *tx)
                .await?;

            for group in user.groups {
                 let _ = GroupRepository::grant(&group, &user.login, &mut *tx)
                     .await?;
            }
        }

        let _ = tx.commit().await?;
        
        return Ok(());
    }
}
