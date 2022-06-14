use log::info;
use text_cleaner::clean::Clean;
use anyhow::{anyhow, Result};
use hashbrown::HashSet;
use yew::{Reducible, UseReducerHandle};


#[derive(Clone, Debug, PartialEq)]
pub struct ProcessorConfigNames {
    pub functions: HashSet<String>,
    pub size: usize
}

impl Default for ProcessorConfigNames {
    fn default() -> Self {
        Self {
            functions: HashSet::from_iter(vec![String::from("trim")].into_iter()),
            size: 1
        }
    }
}


impl ProcessorConfigNames {
    pub fn new() -> Self {
        Self {
            functions: HashSet::new(),
            size: 0
        }
    }
    pub fn all() -> Self {
        Self {
            functions: HashSet::from_iter(vec![
                String::from("trim"),
                String::from("remove_leading_spaces"),
                String::from("remove_trailing_spaces"),
                String::from("remove_empty_lines"),
                String::from("convert_multiple_spaces_to_single"),
                String::from("remove_all_emojis"),
                String::from("remove_all_emails"),
                String::from("remove_all_urls"),
                String::from("remove_non_ascii_characters"),
                String::from("remove_non_alphanumeric_characters"),
                String::from("remove_punctuation_marks"),
                String::from("remove_letter_accents"),
                String::from("normalize_unicode_characters"),
            ].into_iter()),
            size: 13
        }
    }

    pub fn insert(&mut self, function_name: &str) {
        self.functions.insert(String::from(function_name));
        self.size += 1;
    }

    pub fn remove(&mut self, function_name: &str) {
        if self.functions.remove(function_name) {
            self.size -= 1;
        }
    }
}

impl TryFrom<ProcessorConfigNames> for ProcessorConfig {
    type Error = anyhow::Error;

    fn try_from(config_names: ProcessorConfigNames) -> Result<Self, Self::Error> {
        let mut config = ProcessorConfig::new();
        for func in config_names.functions {
            match config.try_add(&func) {
                Ok(()) => {},
                Err(_e) => {panic!();}
            }
        }
        Ok(config)
    }
}

impl TryFrom<&ProcessorConfigNames> for ProcessorConfig {
    type Error = anyhow::Error;

    fn try_from(config_names: &ProcessorConfigNames) -> Result<Self, Self::Error> {
        let mut config = ProcessorConfig::new();
        for func in &config_names.functions {
            match config.try_add(func.as_str()) {
                Ok(()) => {},
                Err(_e) => {panic!();}
            }
        }
        Ok(config)
    }
}


pub fn toggle(hset: &HashSet<String>, key: &str) -> HashSet<String>
{
    let mut hset = hset.clone();
    if hset.contains(key) {
        hset.remove(key);
    } else {
        hset.insert(key.to_string());
    }
    hset
}

impl Reducible for ProcessorConfigNames {
    type Action = String;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {

        let hset = toggle(&self.functions, &action);
        let size = hset.len();
        ProcessorConfigNames {
            functions: hset,
            size
        }.into()
    }
}

pub type ProcessorConfigNamesContext = UseReducerHandle<ProcessorConfigNames>;



pub struct ProcessorConfig {
    functions: Vec<Box<dyn Fn(String) -> String>>,
    size: usize
}

impl ProcessorConfig {
    pub fn new() -> Self {
        Self {
            functions: vec![],
            size: 0
        }
    }

    pub fn chain<F>(&mut self, f: F) 
    where
        F: Fn(String) -> String + 'static
    {
        self.functions.push(Box::new(f));
        self.size += 1;
    }

    pub fn try_add(&mut self, name: &str) -> anyhow::Result<()> {
        let mut exists: bool = true;

        match name {
            "trim" => self.chain(|s| Clean::trim(&s)),
            "remove_leading_spaces" => self.chain(|s| Clean::remove_leading_spaces(&s)),
            "remove_trailing_spaces" => self.chain(|s| Clean::remove_trailing_spaces(&s)),
            "remove_empty_lines" => self.chain(|s| Clean::remove_empty_lines(&s)),
            "convert_multiple_spaces_to_single" => self.chain(|s| Clean::convert_multiple_spaces_to_single(&s)),
            "remove_all_emojis" => self.chain(|s| Clean::remove_all_emojis(&s)),
            "remove_all_emails" => self.chain(|s| Clean::remove_all_emails(&s)),
            "remove_all_urls" => self.chain(|s| Clean::remove_all_urls(&s)),
            "remove_non_ascii_characters" => self.chain(|s| Clean::remove_non_ascii_characters(&s)),
            "remove_non_alphanumeric_characters" => self.chain(|s| Clean::remove_non_alphanumeric_characters(&s)),
            "remove_punctuation_marks" => self.chain(|s| Clean::remove_punctuation_marks(&s)),
            "remove_letter_accents" => self.chain(|s| Clean::remove_letter_accents(&s)),
            "normalize_unicode_characters" => self.chain(|s| Clean::normalize_unicode_characters(&s)),
            _ => {
                exists = false;
            }
        }
        
        match exists {
            true => Ok(()),
            false => Err(anyhow!("Processor {} does not exist", name))
        }
    }
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TextProcessor {
    config: ProcessorConfig
}

impl TextProcessor {
    pub fn new() -> Self {
        Self {
            config: ProcessorConfig::new()
        }
    }
    pub fn with_config_names(config_names: &ProcessorConfigNames) -> anyhow::Result<Self> {
        Ok(Self {
            config: ProcessorConfig::try_from(config_names)?
        })
    }
    pub fn process(&self, text: &str) -> String {
        let mut result = text.to_string();
        for f in &self.config.functions {
            result = f(result.clone());
        }
        result
    }
}