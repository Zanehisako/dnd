use iced::{
    widget::{column, row, text},
    Element,
};

use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, str::FromStr};

pub fn read_json(json_path: &str) -> Vec<Race> {
    let file = File::open(json_path).unwrap();
    let buf = BufReader::new(file);

    let races: Vec<Race> = serde_json::from_reader(buf).unwrap();
    races
}
#[derive(Default, Debug)]
pub struct RacePage {
    race: Option<Race>,
}
#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct Race {
    name: String,
    content: Vec<String>,
    subclass: Option<String>,
    subclass_content: Option<Vec<String>>,
}
impl ToString for Race {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ChosingRace(Race),
    //ChosingSubRace,
}

impl RacePage {
    pub fn new() -> Self {
        RacePage { race: Option::None }
    }

    pub fn view(&self) -> Element<Message> {
        let races = read_json("E:/Rust/dnd/assets/json/races.json");
        let race_picklist: iced::widget::PickList<Race, Vec<Race>, &Race, Message> =
            iced::widget::PickList::new(races, self.race.as_ref(), Message::ChosingRace)
                .placeholder(
                    self.race
                        .clone()
                        .map_or_else(|| "chose a race".to_string(), |s| s.to_string()),
                );
        let col1 = column!(text!("Race"), race_picklist);
        let conent = self
            .race
            .as_ref()
            .map_or_else(|| "".to_string(), |s| s.content.join("\n"));

        let col2 = column!(text!("{}", conent));
        let page = row!(col1, col2).spacing(50).into();

        page
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChosingRace(race) => {
                self.race = Option::from(race);
            }
        }
    }
}
