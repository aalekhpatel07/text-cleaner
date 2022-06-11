use yew::prelude::*;


fn main() {
    yew::start_app::<App>();
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h2 class={"heading"}>{"Hello, World!"}</h2>
        </div>
    }
}