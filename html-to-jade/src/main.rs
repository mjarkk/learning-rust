use std::fs::File;
use std::io;
use std::io::Read;

struct HtmlElArg {
    key: String,
    value: String,
}

struct HtmlEl {
    tag_name: String,
    args: Vec<HtmlElArg>,
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
    pritty_print(out, 0, true);
}

fn pritty_print(el: HtmlEl, mut tabs: usize, is_start: bool) {
    if !is_start {
        let mut to_print = "  ".repeat(tabs) + &el.tag_name;
        if el.args.len() > 0 {
            to_print = to_print + "(";
            for arg in el.args {
                to_print = to_print + &arg.key.to_string();
            }
            to_print = to_print + ")";
        }
        if el.text_contents.len() > 0 {
            to_print = to_print + " " + &el.text_contents;
        }
        println!("{}", to_print);
    }
    for child in el.childeren {
        tabs = tabs + 1;
        if is_start {
            tabs = 0;
        }
        pritty_print(child, tabs, false);
    }
}

enum ParsingStage {
    TagName,
    TagArgName,
    TagArgValue,
    TagEnd,
    Text,
}

fn new_html_el() -> HtmlEl {
    HtmlEl {
        tag_name: String::new(),
        args: vec![],
        childeren: vec![],
        text_contents: String::new(),
    }
}

fn new_html_el_arg() -> HtmlElArg {
    HtmlElArg {
        key: String::new(),
        value: String::new(),
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
            ' ' | '\n' | '\r' | '\t' => adding_noise = true,
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
    let mut parsing_arg = new_html_el_arg();
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
                ' ' => {
                    parsing_state = ParsingStage::TagArgName;
                }
                '/' => {
                    parsing_state = ParsingStage::TagEnd;
                }
                _ => {
                    parsing_el.tag_name.push(current_char);
                }
            },
            ParsingStage::TagArgName => match current_char {
                ' ' => {
                    if parsing_arg.key.len() > 0 {
                        parsing_el.args.push(parsing_arg);
                    }
                    parsing_arg = new_html_el_arg();
                }
                '>' => {
                    if parsing_arg.key.len() > 0 {
                        parsing_el.args.push(parsing_arg);
                    }
                    parsing_arg = new_html_el_arg();
                    parsing_state = ParsingStage::Text;
                }
                '=' | '"' | '\'' => {
                    if parsing_arg.key.len() > 0 {
                        parsing_arg.value.push(current_char);
                        parsing_state = ParsingStage::TagArgValue;
                    } else {
                        parsing_arg.key.push(current_char);
                    }
                }
                _ => {
                    parsing_arg.key.push(current_char);
                }
            },
            ParsingStage::TagArgValue => match current_char {
                '>' | '"' | '\'' => {
                    if parsing_arg.key.len() > 0 {
                        let x: &[_] = &['"', '\''];
                        parsing_arg.value = parsing_arg.value.trim_start_matches(x).to_string();
                        parsing_el.args.push(parsing_arg);
                    }
                    parsing_arg = new_html_el_arg();
                    match current_char {
                        '"' => {
                            parsing_state = ParsingStage::TagArgName;
                        }
                        '>' => {
                            parsing_state = ParsingStage::Text;
                        }
                        _ => {}
                    };
                }
                _ => {
                    parsing_arg.value.push(current_char);
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

                    match chars.split_first() {
                        Some(x) => {
                            if *(x.0) != '/' {
                                let let_to_push = parse_element(chars, false);
                                parsing_el.childeren.push(let_to_push);
                            } else {
                                parsing_state = ParsingStage::TagEnd;
                            }
                        }
                        None => {}
                    }
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
