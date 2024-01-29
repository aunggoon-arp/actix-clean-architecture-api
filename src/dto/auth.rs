#[derive(Debug)]
pub struct AuthPayload {
    pub token: String
}

#[derive(Debug, Serialize)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
}
