use std::fmt::Display;

use http_body_util::BodyExt;
use hyper::Request;
use hyper::client::conn::http1;
use hyper_util::rt::tokio::TokioIo;
use iced::widget::{button, column, pick_list, row, scrollable, text, text_editor, text_input};
use iced::{Element, Task};
use tokio::net::TcpStream;

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

impl From<HttpMethod> for hyper::Method {
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

#[derive(Default)]
pub struct Http {
    title: String,
    url_input: String,
    request_body: text_editor::Content,
    response_text: String,
    selected_http_method: HttpMethod,
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlInputChanged(String),
    RequestBodyChanged(text_editor::Action),

    SendRequest,
    RequestCompleted(Result<String, String>),
    HttpMethodSelected(HttpMethod),
}

impl Http {
    pub fn new(title: String) -> Self {
        Self { title, ..Default::default() }
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            row![
                pick_list(
                    [
                        HttpMethod::Get,
                        HttpMethod::Post,
                        HttpMethod::Put,
                        HttpMethod::Patch,
                        HttpMethod::Delete,
                    ],
                    Some(self.selected_http_method.clone()),
                    Message::HttpMethodSelected,
                ),
                text_input("URL", &self.url_input).on_input(Message::UrlInputChanged),
                button("Send").on_press(Message::SendRequest),
            ],
            text_editor(&self.request_body).on_action(Message::RequestBodyChanged),
            scrollable(text(&self.response_text).size(16))
        ]
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UrlInputChanged(content) => {
                self.url_input = content;
                Task::none()
            },
            Message::RequestBodyChanged(action) => {
                self.request_body.perform(action);
                Task::none()
            },
            Message::RequestCompleted(response) => {
                tracing::info!("Response Received: {} {}", self.selected_http_method, self.url_input);
                match response {
                    Ok(text) => self.response_text = text,
                    Err(e) => self.response_text = format!("ERR: {}", e),
                }
                Task::none()
            },
            Message::SendRequest => {
                tracing::info!("Sending request: {} {}", self.selected_http_method, self.url_input);
                Task::perform(
                    make_request(
                        self.selected_http_method.clone(),
                        self.url_input.clone(),
                        self.request_body.text(),
                    ),
                    |result| result,
                )
            },
            Message::HttpMethodSelected(method) => {
                tracing::info!("Selecting method: {method}");
                self.selected_http_method = method;
                Task::none()
            }
        }
    }
}

async fn make_request(method: HttpMethod, url: String, body: String) -> Message {
    let url = url.parse::<hyper::Uri>().unwrap();

    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await.unwrap();

    let io = TokioIo::new(stream);
    let (mut sender, conn) = http1::handshake::<_, String>(io).await.unwrap();
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            tracing::error!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let path = url.path();
    let req = Request::builder()
        .uri(path)
        .method(method)
        .header(hyper::header::HOST, authority.as_str())
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(body)
        .unwrap();

    let mut res = sender.send_request(req).await.unwrap();

    let mut response_body = Vec::new();
    while let Some(next) = res.frame().await {
        let frame = next.unwrap();
        if let Some(chunk) = frame.data_ref() {
            response_body.extend_from_slice(chunk);
        }
    }

    let response_string = String::from_utf8(response_body).unwrap();
    Message::RequestCompleted(Ok(response_string))
}
