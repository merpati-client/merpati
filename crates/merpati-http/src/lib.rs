use std::fmt::Display;

use iced::widget::{button, column, pick_list, row, text_editor, text_input};
use iced::Length::Fill;
use iced::{Element, Task};

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

#[derive(Default)]
pub struct Http {
    title: String,
    url_input: String,
    request_body: text_editor::Content,
    response_text: text_editor::Content,
    selected_http_method: HttpMethod,
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlInputChanged(String),
    RequestBodyChanged(text_editor::Action),
    ResponseTextChanged(text_editor::Action),

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
            text_editor(&self.response_text).on_action(Message::ResponseTextChanged),
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
            Message::ResponseTextChanged(action) => {
                self.response_text.perform(action);
                Task::none()
            },
            Message::RequestCompleted(response) => {
                tracing::info!("Response Received: {} {}", self.selected_http_method, self.url_input);
                match response {
                    Ok(text) => self.response_text = text_editor::Content::with_text(&text),
                    Err(e) => tracing::error!("Response error: {e:?}"),
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
    let client = reqwest::Client::new();

    let request = client
        .request(reqwest::Method::from(method), &url)
        .header("Content-Type", "application/json")
        .body(body);

    let response = request.send().await.unwrap();
    let response_text = response.text().await.unwrap();

    Message::RequestCompleted(Ok(response_text))
}
