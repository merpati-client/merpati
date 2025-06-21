use std::fmt::Display;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

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

impl Into<reqwest::Method> for HttpMethod {
    fn into(self) -> reqwest::Method {
        match self {
            Self::Get => reqwest::Method::GET,
            Self::Post => reqwest::Method::POST,
            Self::Put => reqwest::Method::PUT,
            Self::Patch => reqwest::Method::PATCH,
            Self::Delete => reqwest::Method::DELETE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpHeaderEntry {
    key: String,
    value: String,
    idle: bool,
}

impl HttpHeaderEntry {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
            idle: false,
        }
    }

    pub fn set_key(&mut self, key: String) {
        if key.is_empty() {
            self.idle = true;
        }

        self.key = key;
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn is_idle(&self) -> bool {
        self.idle
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Clone)]
pub struct HttpHeaders {
    headers: Vec<HttpHeaderEntry>,
}

impl Default for HttpHeaders {
    fn default() -> Self {
        let headers = vec![HttpHeaderEntry::new(
            "Content-Type".to_string(),
            "application/json".to_string(),
        )];

        Self { headers }
    }
}

impl Into<HeaderMap> for HttpHeaders {
    fn into(self) -> HeaderMap {
        let mut header_map = HeaderMap::new();

        for HttpHeaderEntry { key, value, idle } in self.headers {
            if !idle {
                let header_name = HeaderName::from_bytes(key.as_bytes()).unwrap();
                let header_value = HeaderValue::from_str(&value).unwrap();
                header_map.insert(header_name, header_value);
            }
        }

        header_map
    }
}

impl HttpHeaders {
    pub fn iter(&self) -> HttpHeadersIter<'_> {
        HttpHeadersIter {
            inner: self.headers.iter(),
        }
    }
}

pub struct HttpHeadersIter<'a> {
    inner: std::slice::Iter<'a, HttpHeaderEntry>,
}

impl<'a> Iterator for HttpHeadersIter<'a> {
    type Item = &'a HttpHeaderEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub(crate) async fn make_request(
    method: HttpMethod,
    headers: HttpHeaders,
    url: String,
    body: String,
    post_request: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();

    tracing::info!("{:?}", headers);

    let request = client.request(method.into(), &url).headers(headers.into()).body(body);

    let response = request.send().await?;
    merpati_script::post_request(post_request, response.status().as_u16() as usize);
    let response_text = response.text().await?;

    Ok(response_text)
}
