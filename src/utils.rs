use std::collections::HashMap;
use std::str::from_utf8;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "resources/"]
struct Asset;

#[derive(Debug)]
pub struct Resources {
    pub help: String,
    messages: HashMap<String, String>
}

impl Resources {
    pub fn new(help: String, messages: HashMap<String, String>) -> Resources { Resources { help, messages }}

    pub fn init(lang: &str) -> Resources {
        Resources::new(read_resource(lang, "help"), to_map(read_resource(lang, "msg")))
    }
    pub fn get(&self, name: &str) -> Option<&String> {
        self.messages.get(name)
    }
}

fn read_resource(lang: &str, name: &str) -> String {
    let asset = Asset::get(&format!("{}_{}.txt", name, lang)).expect(&format!("No help file for language {}", lang));
    let str = from_utf8(asset.data.as_ref()).expect("Invalid resource file");
    return str.to_owned();
}

fn to_map(str: String) -> HashMap<String, String> {
    str.split("\n").filter(|s| !s.is_empty()).map(
        |s| s.split("  ").collect::<Vec<_>>()).map(|v| (v[0].trim().to_string(), v.last().unwrap().trim().to_string())
    ).collect::<HashMap<_,_>>()
}

#[cfg(test)]
mod tests {
    use crate::utils::Resources;

    #[test]
    fn test_get() {
        let resources = Resources::init("FR");
        assert_eq!(6, resources.messages.len());
        assert_eq!("Le symbole '{1}' n'est pas défini", resources.get("UndefinedSymbol").unwrap())
    }
}
