use iced::{Element, Length, Task, widget::text_editor};
use iced_aw::TabLabel;

use crate::tabs::HttpTab;

#[derive(Debug, Clone)]
pub enum Message {
    ScriptChanged(text_editor::Action),
}

#[derive(Default)]
pub struct Tab {
    pub content: text_editor::Content,
}

impl Tab {
    pub fn update(&mut self, message: Message) -> Task<crate::Message> {
        match message {
            Message::ScriptChanged(action) => {
                self.content.perform(action);
                Task::none()
            },
        }
    }
}

impl HttpTab for Tab {
    type Message = crate::Message;

    fn title(&self) -> String {
        "Script".to_string()
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, _> = text_editor(&self.content)
            .height(Length::Fill)
            .on_action(Message::ScriptChanged)
            .into();

        content.map(crate::Message::ScriptTabMessage)
    }
}
