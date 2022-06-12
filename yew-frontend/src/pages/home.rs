use yew::prelude::*;
use crate::components::*;
use crate::layouts::*;


#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <BaseLayout>
            <Textbox/>
        </BaseLayout>
    }
}