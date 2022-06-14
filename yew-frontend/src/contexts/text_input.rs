
use yew::{UseReducerHandle, Reducible};

#[derive(Debug, Clone, PartialEq)]
pub struct TextInput {
    pub raw: String,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            raw: String::from(""),
        }
    }
}

pub type TextInputContext = UseReducerHandle<TextInput>;

impl Reducible for TextInput {
    type Action = String;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        Self {
            raw: action
        }.into()
    }
}