// use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::spawn_local;
// use web_sys::window;
use text_cleaner_frontend::router::{switch, Route};
use yew::function_component;
use yew_router::{Switch, BrowserRouter};
use yew::prelude::*;
use text_cleaner_frontend::ContextComp;


#[function_component(Base)]
fn base() -> Html {
    html! {
        <ContextComp>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextComp>
    }
}

fn main() {
    yew::start_app::<Base>();
}