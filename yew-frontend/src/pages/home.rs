use crate::components::*;
use crate::layouts::*;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <BaseLayout>
            <Settings/>
        </BaseLayout>
    }
}
