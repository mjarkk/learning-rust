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
}

fn main() {
    let html_data = match load_file() {
        Ok(out) => out,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let out = parse_element(&html_data.chars().collect());

    println!("{}", out.tag_name)
}

fn parse_element(chars: &Vec<char>) -> Vec<HtmlEl> {
    let output:Vec<HtmlEl> = vec![];

    let mut parsing_el = HtmlEl {
        tag_name: String::new(),
        args: vec![],
        childeren: vec![],
    };
    let mut parsing_tag = false;
    let mut parsing_tag_title = false;
    let mut parsing_end = true;

    loop {
        let current_char = match chars.split_first() {
            Some(x) => x.0,
            None => return output,
        };

        match *current_char {
            '<' => {
                parsing_tag = true;
                parsing_tag_title = true;
            }
            '>' => {
                if parsing_tag {
                    parsing_tag = false;
                    parsing_tag_title = false;
                    parsing_end = false;
                    if !parsing_end {
                        el.childeren = parse_element(chars);
                    }
                }
            }
            ' ' => {
                if parsing_tag && parsing_tag_title {
                    parsing_tag_title = false;
                }
            }
            '=' => {}
            '"' => {}
            '\'' => {}
            _ => {
                if parsing_tag && parsing_tag_title {
                    el.tag_name.push(*current_char);
                }
            }
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
