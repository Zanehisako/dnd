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

pub fn parse_txt(text: &str) /*-> Vec<iced::widget::Text>*/
{
    enum State {
        S0,
        S1,
        S2,
        S3,
        S4,
        S5,
        S6,
    }
    let mut current_state = State::S0;
    let mut word = String::new();
    for c in text.chars() {
        match current_state {
            State::S0 => {
                if c.is_alphanumeric() {
                    word.push(c);
                } else {
                    current_state = State::S1;
                }
            }

            State::S1 => {
                if c.is_alphanumeric() {
                    current_state = State::S2;
                } else {
                    current_state = State::S3;
                }
            }

            State::S2 => {
                if c.is_alphanumeric() {
                    word.push(c);
                } else {
                    current_state = State::S4;
                }
            }

            State::S3 => {
                if c.is_alphanumeric() {
                    word.push(c);
                } else {
                    current_state = State::S5;
                }
            }

            State::S4 => {
                current_state = State::S0;
                println!("end");
            }

            State::S5 => {
                if c.is_alphanumeric() {
                    println!("Error!");
                } else {
                    current_state = State::S6;
                }
            }

            State::S6 => {
                current_state = State::S0;
                println!("end");
            }
        }
    }
    println!("{}", word);
    /*let parsed_txt: Vec<iced::widget::Text> = None;
    parsed_txt*/
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
                parse_txt("**Hello**");
                self.race = Option::from(race);
            }
        }
    }
}
