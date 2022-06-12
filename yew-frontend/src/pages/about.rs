use yew::prelude::*;
use crate::layouts::*;


#[function_component(About)]
pub fn about() -> Html {
    html! {
        <BaseLayout>
            {"About route!"}
        </BaseLayout>
    }
}