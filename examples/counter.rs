use iced::widget::container;
use iced::{Element, Length, Task};
use iced_markup::view;

pub fn main() -> iced::Result {
    iced::application(
        || (Counter::default(), Task::none()),
        Counter::update,
        Counter::view,
    )
    .title("Iced Markup Counter")
    .run()
}

/// The state of our counter application.
#[derive(Default)]
struct Counter {
    value: i32,
}

/// The messages that can be triggered by user interaction.
#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Counter {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = view! {
            column ![spacing: 20] {
                button("Increment") ![+click: Message::IncrementPressed] {},

                text(self.value.to_string()) ![size: 50] {},

                if self.value > 10 {
                    text("High Value!") | |_| iced::widget::text::Style { color: Some(iced::Color::from_rgb(1.0, 0.0, 0.0)) } {}
                },

                button("Decrement") ![+click: Message::DecrementPressed] {},
            }
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
