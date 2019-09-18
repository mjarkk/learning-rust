use std::fs::File;
use std::io;
use std::io::Read;

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
            println!("Recived an error: {}", e);
            return;
        }
    };

    let out = parse_element(&mut html_data.chars().collect(), true);
    pritty_print(out, 0);
}

fn pritty_print(el: HtmlEl, tabs: usize) {
    print!("{}{}", "  ".repeat(tabs), el.tag_name);
    if el.text_contents.len() > 0 {
        print!("| {}", el.text_contents);
    }
    println!("");
    for child in el.childeren {
        pritty_print(child, tabs + 1);
    }
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

fn format_html_text(s: String) -> String {
    let mut output = String::new();
    let mut adding_noise = false;
    for c in s.chars() {
        match c {
            ' ' | '\n' => adding_noise = true,
            _ => {
                if adding_noise {
                    output.push(' ');
                    adding_noise = false;
                }
                output.push(c);
            }
        }
    }

    if output == " " {
        output = String::new();
    }
    output
}

fn parse_element(chars: &mut Vec<char>, is_init: bool) -> HtmlEl {
    let mut parsing_el = new_html_el();
    let mut current_text = String::new();
    let mut parsing_state = ParsingStage::TagName;
    if is_init {
        parsing_state = ParsingStage::Text;
    }

    loop {
        let current_char = match chars.split_first() {
            Some(x) => *(x.0),
            None => return parsing_el,
        };
        chars.drain(0..1);

        match parsing_state {
            ParsingStage::TagName => match current_char {
                '>' => {
                    parsing_state = ParsingStage::Text;
                }
                '/' => {
                    parsing_state = ParsingStage::TagEnd;
                }
                _ => {
                    parsing_el.tag_name.push(current_char);
                }
            },
            ParsingStage::TagEnd => match current_char {
                '>' => {
                    return parsing_el;
                }
                _ => {}
            },
            ParsingStage::Text => match current_char {
                '<' => {
                    current_text = format_html_text(current_text);
                    if current_text.len() > 0 {
                        parsing_el.childeren.push(new_text_el(current_text));
                    }
                    current_text = String::new();
                    parsing_el.childeren.push(parse_element(chars, false));
                }
                _ => {
                    current_text.push(current_char);
                }
            },
        };
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
