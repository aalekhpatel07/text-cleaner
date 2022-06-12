pub use text_cleaner::clean::Clean;
// pub use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};

mod components;
mod glue;
pub mod router;
mod layouts;

pub mod pages;
pub mod contexts;
pub use components::*;