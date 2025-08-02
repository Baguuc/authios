use crate::prelude::*;

pub struct UserRepository;

impl UserRepository {
    pub async fn insert<'c, C: sqlx::postgres::PgExecutor<'c>>(login: &String, pwd: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO users (login, pwd) VALUES ($1, $2);";
        let result = query(sql).bind(login).bind(Self::hash_password(pwd.clone())?).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn retrieve<'c, C: sqlx::postgres::PgExecutor<'c>>(login: &String, client: C) -> Result<authios_domain::User> {
        use sqlx::query_as;

        let sql = "SELECT
          u.login,
          u.pwd,
          ARRAY_REMOVE(ARRAY_AGG(ug.group_name), NULL) AS groups
        FROM 
          users u
        LEFT JOIN 
          user_groups ug 
          ON 
          ug.user_login = u.login
        WHERE
            u.login = $1
        GROUP BY
          u.login, u.pwd;
        ;";
        let result = query_as(sql).bind(login).fetch_one(client).await;

        match result {
            Ok(data) => return Ok(data),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn list<'c, C: sqlx::postgres::PgExecutor<'c>>(client: C) -> Result<Vec<authios_domain::User>> {
        use sqlx::query_as;

        let sql = "SELECT
          u.login,
          u.pwd,
          ARRAY_REMOVE(ARRAY_AGG(ug.group_name), NULL) AS groups
        FROM 
          users u
        LEFT JOIN 
          user_groups ug 
          ON 
          ug.user_login = u.login
        GROUP BY
          u.login, u.pwd
        ;";
        let result = query_as(sql).fetch_all(client).await;

        match result {
            Ok(data) => return Ok(data),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn delete<'c, C: sqlx::postgres::PgExecutor<'c>>(login: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM users WHERE login = $1;";
        let result = query(sql).bind(login).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }  
    
    pub async fn update_pwd<'c, C: sqlx::postgres::PgExecutor<'c>>(login: &String, new_pwd: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "UPDATE users SET pwd = $1 WHERE login = $2;";
        let result = query(sql).bind(new_pwd).bind(login).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }  

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
    
    pub async fn register<'c, C: sqlx::postgres::PgExecutor<'c>>(login: &String, pwd: &String, client: C) -> Result<()> { 
        let pwd = Self::hash_password(pwd.clone())?;

        let _ = Self::insert(login, &pwd, client)
            .await?;
        
        return Ok(());
    }
    
    pub async fn login<'c, C: sqlx::postgres::PgExecutor<'c>>(login: &String, pwd: &String, encoding_key: String, client: C) -> Result<String> {
        let user = Self::retrieve(login, client)
            .await?;

        if !Self::verify_password(pwd, &user.pwd) {
            return Err(Error::Generic(String::from("Wrong password")));
        };
        
        let token = Self::generate_jwt(
            user.login,
            (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
            encoding_key
        )?;
        
        return Ok(token);
    }
    
    pub async fn from_token<'c, C: sqlx::postgres::PgExecutor<'c>>(token: &String, encoding_key: &String, client: C) -> Result<authios_domain::User> {
        let claims = Self::get_claims(token, encoding_key)?;
        let user = Self::retrieve(&claims.sub, client)
            .await?;
        
        return Ok(user);
    }
    
    pub fn hash_password(password: String) -> Result<String> {
        use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
        
        let pwd = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = match Argon2::default().hash_password(pwd, &salt) {
            Ok(hash) => hash,
            Err(err) => return Err(Error::Generic(err.to_string())),
        }
        .to_string();

        return Ok(password_hash);
    }
    
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
    
    fn verify_password(password: &String, password_hash: &String) -> bool {
        use argon2::{Argon2, PasswordHash, PasswordVerifier, password_hash::Encoding};

        let password_hash = match PasswordHash::parse(password_hash.as_str(), Encoding::B64) {
            Ok(hash) => hash,
            Err(_) => return false
        };

        let _ = match Argon2::default().verify_password(password.as_bytes(), &password_hash) {
            Ok(_) => return true,
            Err(_) => return false
        };
    }
    
    fn generate_jwt(login: String, expires: usize, key: String) -> Result<String> {
        use jsonwebtoken::{encode, Header, EncodingKey};

        let claims = authios_domain::Claims {
            sub: login,
            exp: expires,
        };

        let encoded = encode(
            &Header::default(),
            &claims, 
            &EncodingKey::from_secret(key.as_ref())
        )?;

        return Ok(encoded);
    }
    
    fn get_claims(token: &String, encoding_key: &String) -> Result<authios_domain::Claims> {
        use jsonwebtoken::{decode, DecodingKey, Validation};
        
        let decoded = decode::<authios_domain::Claims>(
            token,
            &DecodingKey::from_secret(encoding_key.as_ref()),
            &Validation::default()
        )?;

        return Ok(decoded.claims);
    }
}
