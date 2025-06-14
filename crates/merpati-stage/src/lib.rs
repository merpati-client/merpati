use iced::{Alignment, Element, Task};
use iced::widget::{button, column, row, text};

#[derive(Default)]
pub struct Stage {
    selected_tab: usize,
    tabs: Vec<merpati_http::Http>,
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelect(usize),
    TabClose(usize),
    TabNew,

    Http(merpati_http::Message),
}

impl Stage {
    pub fn view(&self) -> Element<'_, Message> {
        let mut tabs: Vec<Element<'_, Message>> = self.tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| tab_button(i, tab.title()))
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
            Message::TabClose(i) => {
                self.tabs.remove(i);
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
            },
        }
    }
}

fn tab_button<'a>(i: usize, label: String) -> Element<'a, Message> {
    let tab_label = text(label);
    let close_button = button(text("X").size(12)).on_press(Message::TabClose(i));
    let tab_content = row![tab_label, close_button].align_y(Alignment::Center).spacing(5);
    let tab_button = button(tab_content).on_press(Message::TabSelect(i));

    tab_button.into()
}
