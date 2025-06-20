use client::HttpMethod;
use iced::widget::{button, column, pick_list, row, text_editor, text_input};
use iced::{Element, Task};

use crate::client::HttpHeaders;

mod client;

#[derive(Default)]
pub struct Http {
    title: String,
    url_input: String,
    request_body: text_editor::Content,
    post_request_script: text_editor::Content,
    response_text: text_editor::Content,
    selected_http_method: HttpMethod,
    headers: HttpHeaders,
}

#[derive(Debug, Clone)]
pub enum Message {
    Noop,
    UrlInputChanged(String),
    RequestBodyChanged(text_editor::Action),
    PostRequestScriptChanged(text_editor::Action),
    ResponseTextChanged(text_editor::Action),

    SendRequest,
    RequestCompleted(String),
    HttpMethodSelected(HttpMethod),
}

impl Http {
    pub fn new(title: String) -> Self {
        Self {
            title,
            ..Default::default()
        }
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
            text_editor(&self.post_request_script).on_action(Message::PostRequestScriptChanged),
            text_editor(&self.response_text).on_action(Message::ResponseTextChanged),
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Noop => Task::none(),
            Message::UrlInputChanged(content) => {
                self.url_input = content;
                Task::none()
            },
            Message::RequestBodyChanged(action) => {
                self.request_body.perform(action);
                Task::none()
            },
            Message::PostRequestScriptChanged(action) => {
                self.post_request_script.perform(action);
                Task::none()
            },
            Message::ResponseTextChanged(action) => {
                self.response_text.perform(action);
                Task::none()
            },
            Message::RequestCompleted(response) => {
                tracing::info!("Response Received: {} {}", self.selected_http_method, self.url_input);
                self.response_text = text_editor::Content::with_text(&response);
                Task::none()
            },
            Message::SendRequest => {
                tracing::info!("Sending request: {} {}", self.selected_http_method, self.url_input);
                Task::perform(
                    client::make_request(
                        self.selected_http_method.clone(),
                        self.headers.clone(),
                        self.url_input.clone(),
                        self.request_body.text(),
                        self.post_request_script.text(),
                    ),
                    |result| {
                        if let Ok(result) = result {
                            Message::RequestCompleted(result)
                        } else {
                            Message::Noop
                        }
                    },
                )
            },
            Message::HttpMethodSelected(method) => {
                tracing::info!("Selecting method: {method}");
                self.selected_http_method = method;
                Task::none()
            },
        }
    }
}
