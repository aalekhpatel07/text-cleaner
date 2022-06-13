use crate::components::InputOutputBox;
use crate::components::Navbar;
use yew::prelude::*;
use yew::{Children, ContextProvider};
use yew_router::BrowserRouter;

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
                <InputOutputBox/>
            </div>
            <div
                class="w-3/4 h-full overflow-hidden"
            >
                <Navbar/>
                <div class="w-full bg-neutral p-8 h-full">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
