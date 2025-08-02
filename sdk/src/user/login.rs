impl crate::Sdk {
    /// # Sdk::login
    ///
    /// log in as a user and get the session token
    /// 
    /// Params:
    /// + login - login of user to log in as
    /// + pwd - password to authenticate the user
    /// 
    /// Errors:
    /// + when the HTTP request cannot be sent to the API (LoginError::HTTP)
    /// + when the url of the request cannot be created (LoginError::Url)
    /// + when the login and password doesn't match (LoginError::Unauthorized)
    /// 
    pub async fn login(&self, params: LoginParams) -> Result<String, LoginError> {
        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("user")
            .map_err(|error| LoginError::Url(error.to_string()))?;

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&params)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(LoginError::Unauthorized)
        }

        let token = response
            .text()
            .await?;
        
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
    HTTP(#[from] reqwest::Error),
    
    #[error("{0}")]
    Url(String),

    #[error("Unauthorized")]
    Unauthorized
}
