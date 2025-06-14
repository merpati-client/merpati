use iced::{Element, Task};
use iced::widget::{button, column, row, text};

#[derive(Default)]
pub struct Stage {
    selected_tab: usize,
    tabs: Vec<merpati_http::Http>,
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelect(usize),
    TabNew,

    Http(merpati_http::Message),
}

impl Stage {
    pub fn view(&self) -> Element<'_, Message> {
        let mut tabs: Vec<Element<'_, Message>> = self.tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| {
                button(text(tab.title())).on_press(Message::TabSelect(i)).into()
            })
            .collect();

        tabs.push(button("+").on_press(Message::TabNew).into());

        match self.tabs.get(self.selected_tab) {
            Some(tab) => column![row(tabs), tab.view().map(Message::Http)].into(),
            _ => row(tabs).into()
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabNew => {
                let title = format!("Request #{}", self.tabs.len() + 1);
                self.tabs.push(merpati_http::Http::new(title));
                Task::none()
            },
            Message::TabSelect(i) => {
                self.selected_tab = i;
                Task::none()
            },
            Message::Http(msg) => {
                let Some(http) = self.tabs.get_mut(self.selected_tab) else {
                    return Task::none()
                };

                http.update(msg).map(Message::Http)
            }
        }
    }
}
