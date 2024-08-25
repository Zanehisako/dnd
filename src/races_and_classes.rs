use std::str::FromStr;

use iced::{widget::column, Element};

#[derive(Debug)]
struct RacePage {
    race: Races,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChosingRace,
    ChosingSubRace,
    ChosingClass,
}

#[derive(Default, Debug)]
pub struct Races {
    name: String,
    description: String,
}
impl Races {
    pub fn new(new_name: String) -> Self {
        Races {
            name: new_name,
            description: String::from_str("").unwrap(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let col = column!().into();
        col
    }
    pub fn update(&mut self, message: Message) {}
}
#[derive(Debug)]
pub enum Classes {
    Variant1,
    Variant2,
}
