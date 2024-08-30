use iced::{
    widget::{column, text},
    Element,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct RacePage {
    race: Option<Races>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Race {
    content: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChosingRace(Races),
    //ChosingSubRace,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Races {
    Dwarf,
    Elf,
    Halfling,
    Human,
    Dragonborn,
    Gnome,
    HalfElf,
    HalfOrc,
    Tiefling,
}

impl ToString for Races {
    fn to_string(&self) -> String {
        match self {
            Self::Dwarf => String::from_str("Dwarf").unwrap(),
            Self::Elf => String::from_str("Elf").unwrap(),
            Self::Halfling => String::from_str("Halfling").unwrap(),
            Self::Human => String::from_str("Human").unwrap(),
            Self::Dragonborn => String::from_str("Dragonborn").unwrap(),
            Self::Gnome => String::from_str("Gnome").unwrap(),
            Self::HalfElf => String::from_str("HalfElf").unwrap(),
            Self::HalfOrc => String::from_str("HalfOrc").unwrap(),
            Self::Tiefling => String::from_str("Tiefling").unwrap(),
        }
    }
}
impl RacePage {
    pub fn new() -> Self {
        RacePage { race: Option::None }
    }

    pub fn view(&self) -> Element<Message> {
        let races_options = vec![
            Races::Dwarf,
            Races::Elf,
            Races::Human,
            Races::Halfling,
            Races::Dragonborn,
            Races::Gnome,
            Races::HalfElf,
            Races::HalfOrc,
            Races::Tiefling,
        ];
        let race_picklist: iced::widget::PickList<Races, Vec<Races>, &Races, Message> =
            iced::widget::PickList::new(races_options, self.race.as_ref(), Message::ChosingRace)
                .placeholder(
                    self.race
                        .map_or_else(|| "chose a race".to_string(), |s| s.to_string()),
                );
        let col = column!(text!("Race"), race_picklist).into();
        col
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChosingRace(race) => {
                self.race = Option::from(race);
            }
        }
    }
}
