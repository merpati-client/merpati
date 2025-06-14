use std::fmt::Display;

use iced::widget::{button, column, pick_list, row, scrollable, text, text_input};
use iced::{Element, Task};
use reqwest::Client;

fn main() -> iced::Result {
    tracing_subscriber::fmt().init();

    tracing::info!("Starting Merpati");
    iced::application(Merpati::title, Merpati::update, Merpati::view)
        .theme(|_| iced::Theme::Dark)
        .run()
}

#[derive(Debug, Default, Clone, PartialEq)]
enum HttpMethod {
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

impl From<&HttpMethod> for reqwest::Method {
    fn from(value: &HttpMethod) -> Self {
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
struct Merpati {
    url_input: String,
    response_text: String,
    selected_http_method: HttpMethod,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String),
    SendRequest,
    RequestCompleted(Result<String, String>),
    HttpMethodSelected(HttpMethod),
}

impl Merpati {
    fn title(&self) -> String {
        "Merpati".to_string()
    }

    fn view(&self) -> Element<'_, Message> {
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
                text_input("URL", &self.url_input).on_input(Message::ContentChanged),
                button("Send").on_press(Message::SendRequest),
            ],
            scrollable(text(&self.response_text).size(16))
        ]
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ContentChanged(content) => {
                self.url_input = content;
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
                    make_request(self.selected_http_method.clone(), self.url_input.clone()),
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

async fn make_request(method: HttpMethod, url: String) -> Message {
    let client = Client::new();
    let method: reqwest::Method = (&method).into();
    let request = client.request(method, &url);

    match request.send().await {
        Ok(response) => {
            match response.text().await {
                Ok(text) => Message::RequestCompleted(Ok(text)),
                Err(e) => Message::RequestCompleted(Err(format!("Failed to read response body: {}", e))),
            }
        }
        Err(e) => Message::RequestCompleted(Err(format!("Failed to fetch URL: {}", e))),
    }
}
