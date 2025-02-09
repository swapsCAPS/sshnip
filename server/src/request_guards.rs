use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let configured_key: String = req
            .rocket()
            .figment()
            .extract_inner("api_key")
            .expect("api_key not configured!");

        match req.headers().get_one("x-api-key") {
            Some(incoming_key) if incoming_key == configured_key => {
                Outcome::Success(ApiKey(incoming_key))
            }
            _ => Outcome::Error((Status::Forbidden, ApiKeyError::Invalid)),
        }
    }
}
