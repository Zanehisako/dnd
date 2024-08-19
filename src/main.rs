use pdf_forms::Form;
use serde::{Deserialize, Serialize};
use std::{/*borrow::Cow,*/ fs::File, io::BufReader};

use iced::*;
use nfd::Response;
//use quick_xml::{events::Event, reader::Reader};

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

#[derive(Default)]
struct CharacterInfo {
    value: i32,
    file_location: String,
}
#[derive(Debug, Clone)]
pub enum Message {
    OpenFile,
    Save,
    Charactername(String),
}
impl CharacterInfo {
    pub fn view(&self) -> iced::widget::Row<Message> {
        // We use a column: a simple vertical layout
        let character_name: &str = "";
        let row = iced::widget::Row::new().push(
            iced::widget::TextInput::new("enter character name", &character_name)
                .on_input(Message::Charactername),
        );

        row
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::OpenFile => {
                let result = nfd::open_file_dialog(None, None).unwrap();
                match result {
                    Response::Okay(file_path) => self.file_location = file_path,
                    Response::OkayMultiple(files) => println!("files{:?}", files),
                    Response::Cancel => println!("User canceld"),
                }
            }
            Message::Save => {
                let mut form = Form::load("E:/Rust/dnd/assets/Character Sheet.pdf")
                    .expect("couldnt read the pdf");
                form.set_text(52, String::from("yassine")).unwrap();
                form.save("modified.pdf").unwrap();
                println!("finished saving the file");
            }
            Message::Charactername(string) => {
                println!("{:?}", string);
            }
        }
    }
}

fn read_json(json_path: &str) {
    let file = File::open(json_path).unwrap();
    let buf = BufReader::new(file);

    let spells: Vec<Spell> = serde_json::from_reader(buf).unwrap();
    for spell in spells {
        println!("{:?}", spell.level);
    }
}

fn main() -> iced::Result {
    //let spells_path = "E:/Rust/dnd/assets/spells.json";
    //read_json(spells_path);
    //read_xml();
    iced::run("DND", CharacterInfo::update, CharacterInfo::view)
}
