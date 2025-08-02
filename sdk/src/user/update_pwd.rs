impl crate::Sdk {
    /// # Sdk::update_pwd
    ///
    /// update user password
    ///
    /// Params:
    /// + token - session token retrieved from login, used to authorize the operation
    /// + pwd - new password to set
    ///
    /// Errors:
    /// + when the HTTP request cannot be sent to the API (UpdatePwdError::HTTP)
    /// + when the url of the request cannot be created (UpdatePwdError::Url)
    /// + when provided token is invalid (UpdatePwdError::Unauthorized) 
    ///
    pub async fn update_pwd(&self, params: UpdatePwdParams) -> Result<(), UpdatePwdError> {
        let result = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("user/pwd");

        let url = match result {
            Ok(url) => url,
            Err(error) => return Err(UpdatePwdError::UrlParse(error.to_string()))
        };

        let client = reqwest::Client::new();
        let response = client
            .patch(url)
            .json(&params)
            .header("Authorization", params.token)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(UpdatePwdError::Unauthorized)
        }
        
        return Ok(());
    }
}

#[derive(serde::Serialize)]
pub struct UpdatePwdParams {
    pub token: String,
    pub pwd: String
}

#[derive(thiserror::Error, Debug)]
pub enum UpdatePwdError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    
    #[error("{0}")]
    UrlParse(String),

    #[error("Unauthorized")]
    Unauthorized
}
