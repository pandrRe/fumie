use std::collections::HashMap;

#[derive(Debug)]
pub struct Options {
    pub options_map: HashMap<String, String>
}

impl Options {
    pub fn new() -> Options {
        let mut options_with_defaults = HashMap::new();
        options_with_defaults.insert(String::from("input"), String::from("repl"));
        options_with_defaults.insert(String::from("--repl"), String::from("per_line"));

        Options { options_map: options_with_defaults }
    }

    pub fn load_options(&mut self, args: Vec<String>) {
        let mut current_setting: String = String::from("input");

        for arg in &args[1..] {
            if arg.starts_with('-') {
                current_setting = arg.clone();
                continue;
            }
            
            self.options_map.insert(current_setting.to_owned(), arg.to_owned());
        }
    }

    pub fn get(&self, option: &str) -> Option<&String> {
        self.options_map.get(option)
    }
}