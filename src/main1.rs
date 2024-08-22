use std::default;

use iced::{
    widget::{column, text},
    Element,
};

#[derive(Default)]
struct App {
    current_view: View,
}

#[derive(Default)]
pub enum View {
    #[default]
    Main,
    CharacterInfoPage,
    RaceAndClassPage,
}
#[derive(Debug, Clone)]
enum Message {
    GoToMain,
    GoToCharacterInfoPage,
    GoToRaceAndClassPage,
}

impl App {
    pub fn new() -> Self {
        App {
            current_view: View::Main,
        }
    }
    pub fn view(&self) -> Element<Message> {
        let content = match self.current_view {
            View::Main => column![text("Main Page")].into(),
            View::CharacterInfoPage => column![text("CharacterInfo Page")].into(),
            View::RaceAndClassPage => column![text("RaceAndClass Page")].into(),
        };
        content
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::GoToMain => self.current_view = View::Main,
            Message::GoToCharacterInfoPage => self.current_view = View::CharacterInfoPage,
            Message::GoToRaceAndClassPage => self.current_view = View::RaceAndClassPage,
        };
    }
}
pub fn main() -> iced::Result {
    //let spells_path = "E:/Rust/dnd/assets/spells.json";
    //read_json(spells_path);
    //read_xml();
    iced::run("DND", App::update, App::view)
}
