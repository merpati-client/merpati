use iced::widget::{button, column, scrollable, text, text_input};
use iced::{Element, Task};

fn main() -> iced::Result {
    iced::application(Http::title, Http::update, Http::view)
        .theme(|_| iced::Theme::CatppuccinMocha)
        .run()
}

#[derive(Default)]
struct Http {
   content: String,
   response_text: String,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String),
    SendRequest,
    RequestCompleted(Result<String, String>),
}

impl Http {
    fn title(&self) -> String {
        "Merpati".to_string()
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text_input("URL", &self.content).on_input(Message::ContentChanged),
            button("Send").on_press(Message::SendRequest),
            scrollable(text(&self.response_text).size(16))
        ]
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ContentChanged(content) => {
                self.content = content;
                Task::none()
            },
            Message::RequestCompleted(response) => {
                match response {
                    Ok(text) => self.response_text = text,
                    Err(e) => self.response_text = format!("ERR: {}", e),
                }
                Task::none()
            },
            Message::SendRequest => {
                println!("Sending request to {}", self.content);
                Task::perform(
                    make_request(self.content.clone()),
                    |result| result,
                )
            },
        }
    }
}

async fn make_request(url: String) -> Message {
    match reqwest::get(&url).await {
        Ok(response) => {
            match response.text().await {
                Ok(text) => Message::RequestCompleted(Ok(text)),
                Err(e) => Message::RequestCompleted(Err(format!("Failed to read response body: {}", e))),
            }
        }
        Err(e) => Message::RequestCompleted(Err(format!("Failed to fetch URL: {}", e))),
    }
}
