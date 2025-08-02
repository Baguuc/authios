use crate::Sdk;

impl Sdk {
    pub async fn authorize(&self, params: AuthorizeParams) -> Result<bool, AuthorizeError> {
        let result = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("authorize/{}", params.permission).as_str());

        let url = match result {
            Ok(url) => url,
            Err(error) => return Err(AuthorizeError::UrlParse(error.to_string()))
        };
        
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("Authorization", params.token.0)
            .send()
            .await?;

        println!("{:?}", response);
        
        if response.status() == 200 {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}

#[derive(serde::Serialize)]
pub struct AuthorizeParams {
    pub token: crate::user::Token,
    pub permission: String
}

#[derive(thiserror::Error, Debug)]
pub enum AuthorizeError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    
    #[error("{0}")]
    UrlParse(String),
}

