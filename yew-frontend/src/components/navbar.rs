use yew::prelude::*;
use yew::{Properties};
use yew_router::prelude::*;
use crate::router::Route;


#[derive(Debug, Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or(vec![true, false])]
    items: Vec<bool>,
}


#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let history = use_history().unwrap();

    let to_home = {
        let history = history.clone();
        Callback::once(move |_| history.push(Route::Home))
    };

    let to_about = {
        let history = history.clone();
        Callback::once(move |_| history.push(Route::About))
    };


    html! {
        <nav>
            <ul>
                <li onclick={to_home}> 
                    {"Home"}
                    // <Link<Route> to={Route::Home}>
                    //     {"Home"} 
                    // </Link<Route>>
                </li>
                <li onclick={to_about}> 
                    {"About"}
                    // <Link<Route> to={Route::About}>
                    //     {"About"} 
                    // </Link<Route>>
                </li>
            </ul>
        </nav>
    }
}