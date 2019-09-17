use std::fs::File;
use std::io;
use std::io::Read;

struct HtmlText {
    text: Vec<char>,
}

// let bad_result: Result<String> = Err();
impl HtmlText {
    fn get_letter(&mut self) -> Option<&char> {
        match self.text.split_first() {
            Some(val) => {
                self.text.drain(0..1);
                return Some(val.0);
            }
            None => {
                return None;
            }
        };
    }
}

// struct HtmlElArg {
//     key: String,
//     value: String,
// }

struct HtmlEl {
    tag_name: String,
    // args: Vec<HtmlElArg>,
    childeren: Vec<HtmlEl>,
    text_contents: String,
}

fn main() {
    let html_data = match load_file() {
        Ok(out) => out,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let out = parse_element(&html_data.chars().collect(), true);

    println!("Tag: {}, Contents: {}", out.tag_name, out.text_contents)
}

enum ParsingStage {
    TagName,
    TagEnd,
    Text,
}

fn new_html_el() -> HtmlEl {
    HtmlEl {
        tag_name: String::new(),
        // args: vec![],
        childeren: vec![],
        text_contents: String::new(),
    }
}

fn new_text_el(text: String) -> HtmlEl {
    let mut text_el = new_html_el();
    text_el.text_contents = text;
    text_el
}

fn parse_element(chars: &Vec<char>, is_init: bool) -> HtmlEl {
    let mut parsing_el = new_html_el();
    // let mut parsing_arg = HtmlElArg {
    //     key: String::new(),
    //     value: String::new(),
    // };
    let mut current_text = String::new();
    let mut parsing_state = ParsingStage::TagName;
    if is_init {
        parsing_state = ParsingStage::Text;
    }

    loop {
        let current_char = match chars.split_first() {
            Some(x) => {
                x.0
            },
            None => return parsing_el,
        };
        chars.drain(0..1);

        match parsing_state {
            ParsingStage::TagName => match *current_char {
                '>' => {
                    parsing_state = ParsingStage::Text;
                }
                '/' => {
                    parsing_state = ParsingStage::TagEnd;
                }
                _ => {
                    parsing_el.tag_name.push(*current_char);
                }
            },
            ParsingStage::TagEnd => match *current_char {
                '>' => {
                    return parsing_el;
                }
                _ => {}
            },
            ParsingStage::Text => match *current_char {
                '<' => {
                    parsing_el.childeren.push(new_text_el(current_text));
                    current_text = String::new();
                    parse_element(chars, false);
                }
                _ => {
                    current_text.push(*current_char);
                }
            },
        };

        // match *current_char {
        // };
    }
}

fn load_file() -> Result<String, io::Error> {
    let mut file = File::open("index.html")?;
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}
