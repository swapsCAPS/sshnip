use super::server;
use rocket::http::{Header, Status};
use rocket::local::blocking::Client;

const ERROR_RESPONSE: &'static str = "{
  \"error\": {
    \"code\": 403,
    \"reason\": \"Forbidden\",
    \"description\": \"The server refused to authorize the request.\"
  }
}";

fn get_api_key(client: &Client) -> String {
    client
        .rocket()
        .figment()
        .extract_inner("api_key")
        .expect("api_key not configured!")
}

#[test]
fn post_wrong() {
    let client = Client::tracked(server()).expect("valid rocket instance");
    let response = client
        .post(uri!(super::post))
        .body("yoooo")
        .header(Header::new("x-api-key", "wrong"))
        .header(Header::new("accept", "application/json"))
        .dispatch();
    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.into_string().unwrap(), ERROR_RESPONSE);
}

#[test]
fn get_wrong() {
    let client = Client::tracked(server()).expect("valid rocket instance");
    let response = client
        .get(uri!(super::get))
        .header(Header::new("x-api-key", "wrong"))
        .header(Header::new("accept", "application/json"))
        .dispatch();
    assert_eq!(response.status(), Status::Forbidden);
    assert_eq!(response.into_string().unwrap(), ERROR_RESPONSE);
}

#[test]
fn post() {
    let client = Client::tracked(server()).expect("valid rocket instance");

    let api_key = get_api_key(&client);

    let response = client
        .post(uri!(super::post))
        .body("yoooo")
        .header(Header::new("x-api-key", api_key))
        .dispatch();
    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.into_string().unwrap(), "clipped");
}

#[test]
fn get() {
    let client = Client::tracked(server()).expect("valid rocket instance");

    let api_key = get_api_key(&client);

    let response = client
        .get(uri!(super::get))
        .header(Header::new("x-api-key", api_key))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "");
}

#[test]
fn post_then_get() {
    let client = Client::tracked(server()).expect("valid rocket instance");

    let clip = "henk";

    let api_key = get_api_key(&client);

    {
        let response = client
            .post(uri!(super::post))
            .header(Header::new("x-api-key", api_key.clone()))
            .body(clip)
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.into_string().unwrap(), "clipped");
    }

    {
        let response = client
            .get(uri!(super::get))
            .header(Header::new("x-api-key", api_key.clone()))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), clip);
    }
}
