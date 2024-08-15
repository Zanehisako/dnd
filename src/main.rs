use serde::{Deserialize, Serialize};
use std::{/*borrow::Cow,*/ fs::File, io::BufReader};

use iced::widget::{button, column, text, Column};
use nfd::Response;
use pdf_forms::{self, Form};
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

#[derive(Default)]
struct Counter {
    value: i32,
    file_location: String,
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    OpenFile,
    OpenPDF,
}
impl Counter {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            button("+").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("-").on_press(Message::DecrementPressed),
            button("Openfile").on_press(Message::OpenFile),
            button("OpenPDF").on_press(Message::OpenPDF),
        ]
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::OpenFile => {
                let result = nfd::open_file_dialog(None, None).unwrap();
                match result {
                    Response::Okay(file_path) => self.file_location = file_path,
                    Response::OkayMultiple(files) => println!("files{:?}", files),
                    Response::Cancel => println!("User canceld"),
                }
            }
            Message::OpenPDF => {
                let mut form = Form::load("E:/Rust/dnd/assets/aadail.pdf").unwrap();
                form.set_text(2, String::from("yassine")).unwrap();
                form.save("modified.pdf").unwrap();

                println!("finished saving the file");
            }
        }
    }
}
/*fn read_xml() {
    let mut reader =
        Reader::from_file("E:/Rust/dnd/assets/134412_Official_and_MercerBrew.xml").unwrap();

    let mut count = 0;

    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"tag1" => println!(
                    "attributes values: {:?}",
                    e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                ),
                b"tag2" => count += 1,
                _ => (),
            },
            Ok(Event::Text(e)) => match e.unescape() {
                Ok(Cow::Borrowed(borrowed_str)) => {
                    let borrowed_txt = borrowed_str.replace("\n", "").replace("\t", "");
                    match borrowed_txt.as_str() {
                        "" => (),
                        _ => txt.push(borrowed_txt),
                    }
                }
                Ok(Cow::Owned(owned_str)) => {
                    //println!("owned{:?}", owned_str);
                    txt.push(e.unescape().unwrap().into_owned());
                }
                Err(_) => {
                    println!("error");
                }
            },

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    for text in txt {
        println!("{:?}", text);
    }
}*/

fn read_json() {
    let file = File::open("E:/Rust/dnd/assets/spells.json").unwrap();
    let buf = BufReader::new(file);
    let spells: Vec<Spell> = serde_json::from_reader(buf).unwrap();
    for spell in spells {
        println!("{:?}", spell.level);
    }
}

fn main() -> iced::Result {
    read_json();
    //read_xml();
    iced::run("DND", Counter::update, Counter::view)
}
