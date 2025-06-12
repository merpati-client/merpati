use iced::Element;
use iced::widget::text_input;

fn main() -> iced::Result {
    iced::application("Merpati", update, view)
        .theme(|_| iced::Theme::CatppuccinMocha)
        .run()
}

#[derive(Default)]
struct State {
   content: String,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String)
}

fn view(state: &State) -> Element<'_, Message> {
    text_input("Type something here...", &state.content)
        .on_input(Message::ContentChanged)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ContentChanged(content) => {
            state.content = content;
        }
    }
}
