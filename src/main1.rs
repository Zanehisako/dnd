use iced::{
    widget::{column, text},
    Element,
};

#[derive(Debug)]
pub enum App {
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
    fn new() -> Self {
        App::Main
    }
    fn view(&self) -> Element<Message> {
        let content = match self {
            App::Main => column![text("Main Page")].into(),
            App::CharacterInfoPage => column![text("CharacterInfo Page")].into(),
            App::RaceAndClassPage => column![text("RaceAndClass Page")].into(),
        };
        content
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::GoToMain => *self = App::Main,
            Message::GoToCharacterInfoPage => *self = App::CharacterInfoPage,
            Message::GoToRaceAndClassPage => *self = App::RaceAndClassPage,
        };
    }
}
