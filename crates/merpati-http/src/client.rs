use std::fmt::Display;
use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

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

#[derive(Debug, Clone)]
pub struct HttpHeaders {
    headers: HashMap<String, String>
}

impl Default for HttpHeaders {
    fn default() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        Self { headers }
    }
}

impl Into<HeaderMap> for HttpHeaders {
    fn into(self) -> HeaderMap {
        let mut header_map = HeaderMap::new();

        for (key, value) in self.headers {
            let header_name = HeaderName::from_bytes(key.as_bytes()).unwrap();
            let header_value = HeaderValue::from_str(&value).unwrap();
            header_map.insert(header_name, header_value);
        }
        
        header_map
    }
}

pub(crate) async fn make_request(
    method: HttpMethod,
    headers: HttpHeaders,
    url: String,
    body: String,
    post_request: String,
) -> Message {
    let client = reqwest::Client::new();

    tracing::info!("{:?}", headers);

    let request = client
        .request(reqwest::Method::from(method), &url)
        .headers(headers.into())
        .body(body);

    let response = request.send().await.unwrap();
    merpati_script::post_request(post_request, response.status().as_u16() as usize);
    let response_text = response.text().await.unwrap();

    Message::RequestCompleted(Ok(response_text))
}
