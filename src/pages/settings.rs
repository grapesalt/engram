use crate::styles;

use iced::{
    Element, Length, Task,
    alignment::Vertical,
    widget::{Space, button, column, container, row, text},
};

#[derive(Debug, Default)]
pub struct SettingsPage;

#[derive(Debug, Clone)]
pub enum Message {
    Back,
}

impl SettingsPage {
    #[allow(dead_code)]
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Back => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let header = row![
            Space::with_width(20),
            text("SETTINGS").size(18),
            Space::with_width(Length::Fill),
            button(text("< BACK >").size(14))
                .on_press(Message::Back)
                .padding(8)
                .style(styles::header_button_style),
        ]
        .align_y(Vertical::Center);

        container(
            column![header, Space::with_height(Length::Fill)]
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(styles::background_style)
        .into()
    }
}
