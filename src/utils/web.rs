/// # retrieve_token_from_request
///
/// Retrieves authorization token from "authorization" header in HTTP request.
/// Returns token as [std::string::String] when token is present and valid and None otherwise.
///
pub fn retrieve_token_from_request(req: actix_web::HttpRequest) -> Option<String> {
    let token = req
        .headers()
        .get("authorization");
    if token.is_none() {
        return None;
    }
    
    let token = token
        .unwrap()
        .to_str();
    if token.is_err() {
        return None;
    }

    let token = token
        .unwrap()
        .to_string()
        .replace("Bearer ", "")
        .to_string();

    return Some(token);
}
