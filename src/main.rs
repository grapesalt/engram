mod app;
mod pages;
mod styles;

use app::{App, AppMessage};
use iced::{Element, Size, Task, Theme, window::Settings};

fn update(state: &mut App, message: AppMessage) -> Task<AppMessage> {
    state.update(message)
}

fn view(state: &App) -> Element<'_, AppMessage> {
    state.view()
}

fn title(state: &App) -> String {
    state.title()
}

fn main() -> iced::Result {
    iced::application(title, update, view)
        .theme(|_| Theme::Light)
        .window_size((1000.0, 700.0))
        .window(Settings {
            min_size: Some(Size::new(800.0, 600.0)),
            ..Default::default()
        })
        .font(styles::FONT_BYTES)
        .font(styles::FONT_BOLD_BYTES)
        .default_font(styles::DEFAULT_FONT)
        .run_with(|| (App::default(), Task::none()))
}
