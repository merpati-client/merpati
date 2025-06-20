use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::Container,
};
use iced_aw::TabLabel;

pub mod body;
pub mod response;
pub mod script;

const TAB_PADDING: u16 = 16;

pub trait HttpTab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        Container::new(self.content())
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
