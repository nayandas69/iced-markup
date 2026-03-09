use iced::widget::Column;
use iced::{Renderer, Theme};
use iced_markup::view;

#[derive(Clone, Debug)]
enum Message {
    Click,
}

#[test]
fn test_advanced_features() {
    let show_more = true;
    let items = vec!["A", "B", "C"];

    let _: Column<'_, Message, Theme, Renderer> = view! {
        column ![spacing: 20] {
            text("Advanced Test") {},

            // Native Control Flow: If
            if show_more {
                text("More info shown") {}
            },

            // Native Control Flow: For
            for item in items {
                text(item) {}
            },

            // Event Shorthand: +click instead of on_press
            button("Save") ![+click: Message::Click] {},

            // Thematic Pipe: | style
            text("Styled") | |_| iced::widget::text::Style::default() {},

            // Component Slots: @name
            button("Slotted") {
                @on_press: Message::Click
            }

            // Mix of static and dynamic
            row {
                text("Static") {},
                if true { text("Dynamic") {} },
            }
        }
    };
}
