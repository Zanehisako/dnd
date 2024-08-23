use std::str::FromStr;

use iced::{widget::column, Element};

#[derive(Debug)]
struct RacePage {
    race: Races,
}

enum Message {
    ChosingRace,
    ChosingSubRace,
    ChosingClass,
}

#[derive(Default, Debug)]
pub struct Races {
    description: String,
}
impl Races {
    pub fn new() -> Self {
        Races {
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
