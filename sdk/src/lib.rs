pub mod user;

#[derive(Clone)]
pub struct Sdk {
    base_url: reqwest::Url
}

impl Sdk {
    pub fn create(base_url: String) -> Result<Self, SdkCreateError> { 
        let base_url = match reqwest::Url::parse(base_url.as_str()) {
            Ok(url) => url,
            Err(error) => return Err(SdkCreateError::UrlParse(error.to_string()))
        };
        
        return Ok(Self { base_url });
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SdkCreateError {
    #[error("{0}")]
    UrlParse(String)
}
