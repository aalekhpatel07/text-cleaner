use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About/> },
        Route::NotFound => html! {<h1>{"Not found"}</h1>},
        // _ => html! {<Home/>}
    }
}