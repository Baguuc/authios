pub enum RootUserCheckPasswordResponse {
    Ok(bool)
}

impl From<bool> for RootUserCheckPasswordResponse {
    fn from(b: bool) -> Self {
        Self::Ok(b)
    }
}

impl Into<actix_web::HttpResponse> for RootUserCheckPasswordResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok(authorized) => HttpResponse::Ok()
                .json(json!({ "code": "ok", "authorized": authorized })),
        }
    }
}
