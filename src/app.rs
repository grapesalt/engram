use crate::pages::{
    home::HomePage, results::ResultsPage, settings::SettingsPage,
};
use iced::{Element, Task};

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Home,
    Settings,
    SearchResults,
}
pub struct App {
    current_page: Page,
    home_page: HomePage,
    settings_page: SettingsPage,
    results_page: ResultsPage,

    pub indexed_files: usize,
    pub indexed_size: String,
}

impl App {
    pub fn title(&self) -> String {
        match self.current_page {
            Page::Home => "Engram - Home".to_string(),
            Page::Settings => "Engram - Settings".to_string(),
            Page::SearchResults => {
                format!("Engram - Results for \"{}\"", self.results_page.query)
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_page: Page::Home,
            home_page: HomePage::default(),
            settings_page: SettingsPage::default(),
            results_page: ResultsPage::default(),
            indexed_files: 152,
            indexed_size: "251GB".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    HomeMessage(crate::pages::home::Message),
    SettingsMessage(crate::pages::settings::Message),
    ResultsMessage(crate::pages::results::Message),
}

impl App {
    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::HomeMessage(msg) => match msg {
                crate::pages::home::Message::NavigateToSettings => {
                    self.current_page = Page::Settings;
                    Task::none()
                }
                crate::pages::home::Message::TriggerReIndex => Task::none(),
                crate::pages::home::Message::SearchSubmit => {
                    let query = self.home_page.search_query.clone();
                    self.current_page = Page::SearchResults;
                    self.results_page.query = query;
                    Task::none()
                }
                crate::pages::home::Message::SearchChanged(_)
                | crate::pages::home::Message::OpenOptions => {
                    self.home_page.update(msg).map(AppMessage::HomeMessage)
                }
            },
            AppMessage::SettingsMessage(msg) => match msg {
                crate::pages::settings::Message::Back => {
                    self.current_page = Page::Home;
                    Task::none()
                }
            },
            AppMessage::ResultsMessage(msg) => match msg {
                crate::pages::results::Message::Back => {
                    self.current_page = Page::Home;
                    Task::none()
                }
            },
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        match self.current_page {
            Page::Home => {
                self.home_page.view(self).map(AppMessage::HomeMessage)
            }
            Page::Settings => {
                self.settings_page.view().map(AppMessage::SettingsMessage)
            }
            Page::SearchResults => {
                self.results_page.view().map(AppMessage::ResultsMessage)
            }
        }
    }
}
