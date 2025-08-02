impl crate::Sdk {
    /// # Sdk::get_info
    ///
    /// get info about a user by token
    ///
    /// Params:
    /// + token - session token retrieved from login, used to authorize the operation
    ///
    /// Errors:
    /// + when the HTTP request cannot be sent to the API (InfoError::HTTP)
    /// + when the url of the request cannot be created (InfoError::Url)
    /// + when provided token is invalid (InfoError::Unauthorized)
    ///
    pub async fn get_info(&self, params: InfoParams) -> Result<authios_domain::User, InfoError> {
        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("user")
            .map_err(|error| InfoError::Url(error.to_string()))?;

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("Authorization", params.token)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(InfoError::Unauthorized)
        }

        let user: authios_domain::User = response
            .json()
            .await?;
        
        return Ok(user);
    }
}

#[derive(serde::Serialize)]
pub struct InfoParams {
    pub token: String
}

#[derive(thiserror::Error, Debug)]
pub enum InfoError {
    #[error(transparent)]
    HTTP(#[from] reqwest::Error),
    
    #[error("{0}")]
    Url(String),

    #[error("Unauthorized")]
    Unauthorized
}
