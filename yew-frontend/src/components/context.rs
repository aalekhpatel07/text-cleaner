use yew::prelude::*;
use yew::{Children, ContextProvider};
use crate::contexts::*;


#[derive(Debug, Properties, PartialEq)]
pub struct ContextProps {
    pub children: Children
}

#[function_component(ContextComp)]
pub fn context_comp(props: &ContextProps) -> Html {
    let theme = use_state(|| Theme::dark());

    html! {
            <ContextProvider<Theme> context={(*theme).clone()}>
                <div class="h-full w-full bg-red-800">
                    { for props.children.iter() }
                </div>    
            </ContextProvider<Theme>>
    }
}
