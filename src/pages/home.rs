use crate::app::App;
use crate::styles;

use iced::{
    Alignment, Element, Length, Task,
    alignment::{Horizontal, Vertical},
    widget::{Space, button, column, container, row, text, text_input},
};

#[derive(Debug, Default)]
pub struct HomePage {
    pub search_query: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    SearchSubmit,
    OpenOptions,
    NavigateToSettings,
    TriggerReIndex,
}

impl HomePage {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchChanged(query) => {
                self.search_query = query;
                Task::none()
            }
            Message::SearchSubmit => Task::none(),
            Message::OpenOptions => Task::none(),
            Message::NavigateToSettings | Message::TriggerReIndex => {
                Task::none()
            }
        }
    }

    pub fn view(&self, app: &App) -> Element<'_, Message> {
        let logo = r#"
    _______   ____________  ___    __  ___
   / ____/ | / / ____/ __ \/   |  /  |/  /
  / __/ /  |/ / / __/ /_/ / /| | / /|_/ / 
 / /___/ /|  / /_/ / _, _/ ___ |/ /  / /  
/_____/_/ |_/\____/_/ |_/_/  |_/_/  /_/   
                                          "#;

        let header = row![
            Space::with_width(Length::Fill),
            button(text("< SETTINGS >").size(14))
                .on_press(Message::NavigateToSettings)
                .padding(8)
                .style(styles::header_button_style),
            Space::with_width(20),
            button(text("< RE-INDEX >").size(14))
                .on_press(Message::TriggerReIndex)
                .padding(8)
                .style(styles::header_button_style),
        ]
        .align_y(Vertical::Top);

        let logo_section = column![
            Space::with_height(50),
            text(logo)
                .size(16)
                .font(styles::DEFAULT_BOLD_FONT)
                .align_x(Horizontal::Center)
                .width(Length::Fill),
            Space::with_height(10),
            text("[ LOCAL CLIP SEARCH ]")
                .size(16)
                .align_x(Horizontal::Center)
                .width(Length::Fill),
            Space::with_height(40),
        ]
        .align_x(Alignment::Center);

        let search_section = column![
            container(
                text_input("SEARCH....", &self.search_query)
                    .on_input(Message::SearchChanged)
                    .on_submit(Message::SearchSubmit)
                    .padding(15)
                    .size(18)
                    .width(Length::Fixed(640.0))
                    .style(styles::search_input_style)
            )
            .center_x(Length::Fill),
            Space::with_height(20),
            container(
                row![
                    Space::with_width(Length::Fill),
                    button(text("OPTIONS").size(14))
                        .on_press(Message::OpenOptions)
                        .padding([12, 24])
                        .style(styles::body_button_style),
                    Space::with_width(20.0),
                    button(text("GO").size(14))
                        .on_press(Message::SearchSubmit)
                        .padding([12, 24])
                        .style(styles::body_button_style),
                ]
                .width(Length::Fixed(640.0))
            )
            .center_x(Length::Fill),
        ];

        let footer = row![
            text(format!(
                "[ INDEXED: {}, SIZE: {} ]",
                app.indexed_files, app.indexed_size
            ))
            .size(14),
            Space::with_width(Length::Fill),
            text("[ MADE BY GRAPESALT ]").size(14),
        ]
        .padding(20);

        container(
            column![
                header,
                logo_section,
                search_section,
                Space::with_height(Length::Fill),
                footer,
            ]
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
