use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! {<h1>{"Not found"}</h1>},
        // _ => html! {<Home/>}
    }
}