use crate::user::{UserSdk,Token};

impl UserSdk {
    pub async fn login(&self, params: LoginParams) -> Result<Token, LoginError> {
        let result = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("user");

        let url = match result {
            Ok(url) => url,
            Err(error) => return Err(LoginError::UrlParse(error.to_string()))
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&params)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(LoginError::Unauthorized)
        }

        let response_body = response
            .text()
            .await?;
        
        let token = crate::user::Token(response_body);
        
        return Ok(token);
    }
}

#[derive(serde::Serialize)]
pub struct LoginParams {
    pub login: String,
    pub pwd: String
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    
    #[error("{0}")]
    UrlParse(String),

    #[error("Unauthorized")]
    Unauthorized
}
