use std::fmt::Display;

use crate::Message;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum HttpMethod {
    #[default]
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
        })
    }
}

impl From<HttpMethod> for reqwest::Method {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::Get => Self::GET,
            HttpMethod::Post => Self::POST,
            HttpMethod::Put => Self::PUT,
            HttpMethod::Patch => Self::PATCH,
            HttpMethod::Delete => Self::DELETE,
        }
    }
}

pub(crate) async fn make_request(method: HttpMethod, url: String, body: String, post_request: String) -> Message {
    let client = reqwest::Client::new();

    let request = client
        .request(reqwest::Method::from(method), &url)
        .header("Content-Type", "application/json")
        .body(body);

    let response = request.send().await.unwrap();
    merpati_script::post_request(post_request, response.status().as_u16() as usize);
    let response_text = response.text().await.unwrap();

    Message::RequestCompleted(Ok(response_text))
}
