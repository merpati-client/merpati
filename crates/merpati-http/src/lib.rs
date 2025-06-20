use client::HttpMethod;
use iced::widget::{button, column, pick_list, row, text_editor, text_input};
use iced::{Element, Task};
use iced_aw::Tabs;

use crate::client::HttpHeaders;
use crate::tabs::{HttpTab, body, response, script};

mod client;
mod tabs;

#[derive(Default)]
pub struct Http {
    title: String,
    url_input: String,
    selected_http_method: HttpMethod,
    headers: HttpHeaders,

    active_tab: TabId,
    body_tab: body::Tab,
    response_tab: response::Tab,
    script_tab: script::Tab,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum TabId {
    #[default]
    Body,
    Response,
    Script,
}

#[derive(Debug, Clone)]
pub enum Message {
    Noop,

    SendRequest,
    RequestCompleted(String),
    HttpMethodSelected(HttpMethod),
    UrlInputChanged(String),

    TabSelected(TabId),
    BodyTabMessage(body::Message),
    ResponseTabMessage(response::Message),
    ScriptTabMessage(script::Message),
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
        let tabs: Element<'_, Message> = Tabs::new(Message::TabSelected)
            .push(TabId::Body, self.body_tab.tab_label(), self.body_tab.view())
            .push(TabId::Response, self.response_tab.tab_label(), self.response_tab.view())
            .push(TabId::Script, self.script_tab.tab_label(), self.script_tab.view())
            .set_active_tab(&self.active_tab)
            .into();

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
            tabs
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
            Message::RequestCompleted(response) => {
                tracing::info!("Response Received: {} {}", self.selected_http_method, self.url_input);
                self.response_tab.content = text_editor::Content::with_text(&response);
                Task::none()
            },
            Message::SendRequest => {
                tracing::info!("Sending request: {} {}", self.selected_http_method, self.url_input);
                Task::perform(
                    client::make_request(
                        self.selected_http_method.clone(),
                        self.headers.clone(),
                        self.url_input.clone(),
                        self.body_tab.content.text(),
                        self.script_tab.content.text(),
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

            Message::TabSelected(tab_id) => {
                tracing::info!("Tab selected: {:?}", tab_id);
                self.active_tab = tab_id;
                Task::none()
            },

            Message::BodyTabMessage(msg) => self.body_tab.update(msg),
            Message::ResponseTabMessage(msg) => self.response_tab.update(msg),
            Message::ScriptTabMessage(msg) => self.script_tab.update(msg),
        }
    }
}
