use pdf_forms::Form;
use serde::{Deserialize, Serialize};
use std::{/*borrow::Cow,*/ borrow::Borrow, fs::File, io::BufReader, str::FromStr, u8};

#[derive(Serialize, Deserialize, Debug)]
struct Spell {
    name: Option<String>,
    desc: Option<String>,
    page: Option<String>,
    range: Option<String>,
    components: Option<String>,
    ritual: Option<String>,
    material: Option<String>,
    circles: Option<String>,
    higher_level: Option<String>,
    patrons: Option<String>,
    duration: Option<String>,
    concentration: Option<String>,
    school: Option<String>,
    casting_time: Option<String>,
    level: Option<String>,
    class: Option<String>,
    oaths: Option<String>,
    archetype: Option<String>,
    domains: Option<String>,
    weight: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Races {
    name: Option<String>,
    desc: Option<String>,
    page: Option<String>,
    range: Option<String>,
    components: Option<String>,
    ritual: Option<String>,
    material: Option<String>,
    circles: Option<String>,
    higher_level: Option<String>,
    patrons: Option<String>,
    duration: Option<String>,
    concentration: Option<String>,
    school: Option<String>,
    casting_time: Option<String>,
    level: Option<String>,
    class: Option<String>,
    oaths: Option<String>,
    archetype: Option<String>,
    domains: Option<String>,
    weight: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Sex {
    Male,
    Female,
}
impl ToString for Sex {
    fn to_string(&self) -> String {
        match self {
            Self::Male => String::from_str("Male").unwrap(),
            Self::Female => String::from_str("Female").unwrap(),
        }
    }
}
impl Borrow<str> for Sex {
    fn borrow(&self) -> &str {
        match self {
            Self::Male => "Male",
            Self::Female => "female",
        }
    }
}

#[derive(Default)]
pub struct CharacterInfo {
    name: String,
    level: Option<u8>,
    sex: Option<Sex>,
    feats: bool,
    multiclassing: bool,
}
#[derive(Debug, Clone)]
pub enum Message {
    Save,
    Charactername(String),
    ChosingSex(Sex),
    ChosingLevel(u8),
    Tooglefeat(bool),
    ToogleMulticlassing(bool),
    NextPage,
}
impl CharacterInfo {
    pub fn new() -> Self {
        CharacterInfo {
            name: String::from_str("").unwrap(),
            level: Option::None,
            sex: Option::None,
            feats: false,
            multiclassing: false,
        }
    }
    pub fn view(&self) -> iced::Element<Message> {
        // We use a column: a simple vertical layout
        let row = iced::widget::row!(
            iced::widget::text("Character name"),
            iced::widget::TextInput::new("Enter name", &self.name).on_input(Message::Charactername)
        )
        .spacing(20)
        .padding(50);
        let sex_options = vec![Sex::Male, Sex::Female];
        let sex_picklist: iced::widget::PickList<Sex, Vec<Sex>, &Sex, Message> =
            iced::widget::PickList::new(sex_options, self.sex.as_ref(), Message::ChosingSex)
                .placeholder(
                    self.sex
                        .map_or_else(|| "Select sex".to_string(), |s| s.to_string()),
                );
        let row2: iced::widget::Row<Message> =
            iced::widget::row!(iced::widget::text("sex"), sex_picklist)
                .spacing(20)
                .padding(50);
        let levels: Vec<u8> = (1..21).map(|u| u as u8).collect();
        let starting_level: iced::widget::PickList<u8, Vec<u8>, u8, Message> =
            iced::widget::PickList::new(levels, self.level, Message::ChosingLevel).placeholder(
                self.level
                    .map_or_else(|| "Select level".to_string(), |s| s.to_string()),
            );
        let row3: iced::widget::Row<Message> =
            iced::widget::row!(iced::widget::text("Starting level"), starting_level)
                .spacing(20)
                .padding(50);
        let feat_toggle: iced::widget::Checkbox<Message> =
            iced::widget::checkbox("feats", self.feats).on_toggle(Message::Tooglefeat);
        let row4: iced::widget::Row<Message> =
            iced::widget::row!(feat_toggle).spacing(20).padding(50);
        let multiclassing_toggle: iced::widget::Checkbox<Message> =
            iced::widget::checkbox("multiclassing", self.multiclassing)
                .on_toggle(Message::ToogleMulticlassing);
        let row5: iced::widget::Row<Message> = iced::widget::row!(multiclassing_toggle)
            .spacing(20)
            .padding(50);
        let col = iced::widget::column!(row, row2, row3, row4, row5);
        col.into()
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Save => {
                let mut form = Form::load("E:/Rust/dnd/assets/Character Sheet.pdf")
                    .expect("couldnt read the pdf");
                form.set_text(52, String::from("yassine")).unwrap();
                form.save("modified.pdf").unwrap();
                println!("finished saving the file");
            }
            Message::Charactername(string) => {
                self.name = string;
            }
            Message::ChosingSex(newsex) => {
                self.sex = Option::from(newsex);
                println!("{:?}", newsex);
            }
            Message::ChosingLevel(level) => {
                self.level = Option::from(level);
                println!("{:?}", level);
            }
            Message::Tooglefeat(bool) => {
                self.feats = bool;
            }
            Message::ToogleMulticlassing(bool) => {
                self.multiclassing = bool;
            }
            Message::NextPage => {}
        }
    }
}
