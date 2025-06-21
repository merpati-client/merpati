use iced::widget::{Row, button, column, text_input};
use iced::{Element, Task};
use iced_aw::TabLabel;

use crate::client::HttpHeaderEntry;
use crate::{client::HttpHeaders, tabs::HttpTab};

#[derive(Debug, Clone)]
pub enum Message {
    KeyChanged(usize, String),
    ValueChanged(usize, String),
    NewHeader,
}

#[derive(Default)]
pub struct Tab {
    pub headers: HttpHeaders,
}

impl Tab {
    pub fn update(&mut self, message: Message) -> Task<crate::Message> {
        match message {
            Message::KeyChanged(i, key) => {
                if let Some(header) = self.headers.get_mut(i) {
                    header.set_key(key);
                }
                Task::none()
            },
            Message::ValueChanged(i, value) => {
                if let Some(header) = self.headers.get_mut(i) {
                    header.set_value(value);
                }
                Task::none()
            },
            Message::NewHeader => {
                self.headers.insert_empty();
                Task::none()
            },
        }
    }
}

impl HttpTab for Tab {
    type Message = crate::Message;

    fn title(&self) -> String {
        "Headers".to_string()
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let mut children: Vec<Element<'_, crate::Message>> = self
            .headers
            .iter()
            .enumerate()
            .map(header_component)
            .collect::<Vec<_>>();

        children.push(
            button("New")
                .on_press(crate::Message::HeadersTabMessage(Message::NewHeader))
                .into(),
        );

        column(children).into()
    }
}

fn header_component((i, h): (usize, &HttpHeaderEntry)) -> Element<'_, crate::Message> {
    Row::new()
        .push(
            text_input("Key", &h.key())
                .on_input(move |key| crate::Message::HeadersTabMessage(Message::KeyChanged(i, key))),
        )
        .push(
            text_input("Value", &h.value())
                .on_input(move |value| crate::Message::HeadersTabMessage(Message::ValueChanged(i, value))),
        )
        .into()
}
