mod character_info;
mod races_and_classes;

use iced::{
    widget::{button, column, container, text},
    Element,
};
#[derive(Default)]
struct App {
    current_view: View,
    character_info: character_info::CharacterInfo,
    races_and_classes: races_and_classes::RacePage,
}

#[derive(Default, Debug, Clone)]
pub enum View {
    #[default]
    Main,
    CharacterInfoPage,
    RaceAndClassPage,
}
#[derive(Debug, Clone)]
enum Message {
    SwitchView(View),
    GoToCharacterInfoPage(character_info::Message),
    GoToRaceAndClassPage(races_and_classes::Message),
}

impl App {
    pub fn view(&self) -> Element<Message> {
        let content = match self.current_view {
            View::Main => column![text("Main Page")].into(),
            View::CharacterInfoPage => self
                .character_info
                .view()
                .map(Message::GoToCharacterInfoPage),
            View::RaceAndClassPage => self
                .races_and_classes
                .view()
                .map(Message::GoToRaceAndClassPage),
        };
        let row: iced::widget::Row<Message> = iced::widget::row!(
            button("Go to CharacterInfoPage")
                .on_press(Message::SwitchView(View::CharacterInfoPage)),
            button("Go to Main Page").on_press(Message::SwitchView(View::Main)),
            button("Go to Races").on_press(Message::SwitchView(View::RaceAndClassPage)),
        )
        .padding(50)
        .spacing(50);

        container(column!(content, row).padding(50).spacing(50)).into()
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SwitchView(view) => self.current_view = view,
            Message::GoToCharacterInfoPage(msg) => self.character_info.update(msg),
            Message::GoToRaceAndClassPage(msg) => self.races_and_classes.update(msg),
        };
    }
}
pub fn main() -> iced::Result {
    //let spells_path = "E:/Rust/dnd/assets/spells.json";
    //character_info::read_json(spells_path);
    iced::run("DND", App::update, App::view)
}
