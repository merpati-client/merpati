use iced::widget::{button, row, text_input, Row};

fn main() -> iced::Result {
    iced::application("Merpati", Http::update, Http::view)
        .theme(|_| iced::Theme::CatppuccinMocha)
        .run()
}

#[derive(Default)]
struct Http {
   content: String,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String),
    SendRequest,
}

impl Http {
    fn view(&self) -> Row<'_, Message> {
        row![
            text_input("URL", &self.content)
                .on_input(Message::ContentChanged),
            button("Send").on_press(Message::SendRequest),
        ]
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ContentChanged(content) => self.content = content,
            Message::SendRequest => println!("Sending request to {}", self.content),
        }
    }
}
