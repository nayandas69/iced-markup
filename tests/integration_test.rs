use iced::widget::Column;
use iced::{Renderer, Theme};
use iced_markup::view;

#[test]
fn test_basic_expansion() {
    let _: Column<'_, &str, Theme, Renderer> = view! {
        column ![spacing: 20] {
            text("Hello") {},
            button("Click") ![on_press: "dummy"] {},
        }
    };
}

#[test]
fn test_nested_structure() {
    let _: Column<'_, (), Theme, Renderer> = view! {
        column {
            row ![spacing: 10] {
                text("Left") {},
                text("Right") {},
            },
        }
    };
}
