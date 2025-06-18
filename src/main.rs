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
    Stage(merpati_stage::Stage),
}

struct Merpati {
    screens: Screens,
}

impl Default for Merpati {
    fn default() -> Self {
        Self {
            screens: Screens::Stage(merpati_stage::Stage::default()),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Stage(merpati_stage::Message),
}

impl Merpati {
    fn title(&self) -> String {
        "Merpati".to_string()
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.screens {
            Screens::Stage(stage) => stage.view().map(Message::Stage),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Stage(msg) => {
                let Screens::Stage(stage) = &mut self.screens;
                stage.update(msg).map(Message::Stage)
            },
        }
    }
}
