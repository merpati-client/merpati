use iced::{widget::text, Element, Task};
use iced_aw::TabLabel;

use crate::{client::HttpHeaders, tabs::HttpTab};

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Default)]
pub struct Tab {
    pub headers: HttpHeaders,
}

impl Tab {
    pub fn update(&mut self, _message: Message) -> Task<crate::Message> {
        Task::none()
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
        text("Not Implemented").into()
    }
}
