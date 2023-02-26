use convert_case::Casing;
use regex::Regex;



use convert_case::{Case};

use crate::list::ListField;

pub fn parse_list_file(content: &str) {
    let dashed_module_name = list_module_name(content).unwrap();
    let between_curly = get_between_curly(content).unwrap();

    let fields = between_curly.lines().map(|l| l.split(":").collect::<Vec<_>>()[0]).collect::<String>();

    println!("{}", dashed_module_name);
    println!("{}", between_curly);


}

pub fn list_module_name(content: &str) -> Option<String> {
    let re = Regex::new(r"export type (?P<module_name>\w+)ListModel").unwrap();
    let module_name = re.captures(content).and_then(|cap| cap.name("module_name").map(|m| m.as_str()));

    match &module_name {
        Some(m) => Some(m.to_case(Case::Kebab)),
        None => None
    }
}

pub fn get_between_curly(content: &str) -> Option<&str> {
    let re = Regex::new(r"\{(?ms)(?P<between>.+)\}").unwrap();

    re.captures(content).and_then(|cap| cap.name("between").map(|m| m.as_str()))
}

pub fn parse_line(line: &str) -> Option<ListField> {
    let splits: Vec<&str> = line.split(':').collect();

    if splits.len() != 2 {
        return None
    }

    let field = splits[0].trim();
    let mut value = splits[1].trim();

    if value.ends_with(";") {
        let mut chars = value.chars();
        chars.next_back();
        value = chars.as_str();
    }

    

    None
}