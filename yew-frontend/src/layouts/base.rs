use yew::prelude::*;
use yew::{Children, ContextProvider};
use yew_router::BrowserRouter;
use crate::components::Navbar;
use crate::contexts::Theme;


#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub children: Children
}

#[function_component(BaseLayout)]
pub fn base_layout(props: &Props) -> Html {
    let theme = use_context::<Theme>().unwrap();

    html! {
        <div class="flex">
            <div 
                class="bg-blue-400 w-1/5 min-h-screen border-r border-gray-800"
            >
                <Navbar/>
            </div>
            <div
                class="w-4/5 bg-yellow-300 min-h-screen"
            >
                { for props.children.iter() }
            </div>
        </div>
    }
}
