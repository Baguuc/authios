use crate::user::UserSdk;

impl UserSdk {
    pub async fn get_info(&self, params: InfoParams) -> Result<authin_domain::User, InfoError> {
        let result = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("user");

        let url = match result {
            Ok(url) => url,
            Err(error) => return Err(InfoError::UrlParse(error.to_string()))
        };

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("Authorization", params.token.0)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(InfoError::Unauthorized)
        }

        let user: authin_domain::User = response
            .json()
            .await?;
        
        return Ok(user);
    }
}

#[derive(serde::Serialize)]
pub struct InfoParams {
    pub token: crate::user::Token
}

#[derive(thiserror::Error, Debug)]
pub enum InfoError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    
    #[error("{0}")]
    UrlParse(String),

    #[error("Unauthorized")]
    Unauthorized
}
