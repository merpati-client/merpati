use iced::{Element, Font, Task};

fn main() -> iced::Result {
    tracing_subscriber::fmt().init();

    tracing::info!("Starting Merpati");
    iced::application(Merpati::title, Merpati::update, Merpati::view)
        .theme(|_| iced::Theme::Dark)
        .font(include_bytes!("../assets/fonts/geist-mono.ttf"))
        .default_font(Font::with_name("Geist Mono"))
        .run()
}

enum Screens {
    Http(merpati_http::Http)
}

struct Merpati {
    screens: Screens,
}

impl Default for Merpati {
    fn default() -> Self {
        Self {
            screens: Screens::Http(merpati_http::Http::default())
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Http(merpati_http::Message),
}

impl Merpati {
    fn title(&self) -> String {
        "Merpati".to_string()
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.screens {
            Screens::Http(http) => http.view().map(Message::Http),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Http(msg) => {
                let Screens::Http(http) = &mut self.screens;
                http.update(msg).map(Message::Http)
            }
        }
    }
}
