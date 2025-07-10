pub mod login;
pub mod info;
pub mod authorize;
pub mod update_pwd;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Token(pub String);

#[derive(Clone)]
pub struct UserSdk {
    pub base_url: reqwest::Url
}
