use crate::components::InputOutputBox;
use crate::components::Navbar;
use yew::prelude::*;
use yew::{Children};


#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(BaseLayout)]
pub fn base_layout(props: &Props) -> Html {

    html! {
        <div class="flex h-screen space-x-8">
            <div
                class="w-1/4 overflow-hidden"
            >
                <div class="w-full bg-neutral p-8 h-full">
                    { for props.children.iter() }
                </div>
            </div>
            <div
                class="w-3/4 h-full overflow-auto"
            >
                <Navbar/>
                <InputOutputBox/>
            </div>
        </div>
    }
}
