use yew::prelude::*;
use yew::{Children, ContextProvider};
use crate::contexts::*;


#[derive(Debug, Properties, PartialEq)]
pub struct ContextProps {
    pub children: Children
}

#[function_component(ContextComp)]
pub fn context_comp(props: &ContextProps) -> Html {
    let text = use_reducer(TextInput::new);
    let config = use_reducer(ProcessorConfigNames::new);

    html! {
        <ContextProvider<ProcessorConfigNamesContext> context={config}>
            <ContextProvider<TextInputContext> context={text}>
                <div class="h-full w-full">
                    { for props.children.iter() }
                </div>    
            </ContextProvider<TextInputContext>>
        </ContextProvider<ProcessorConfigNamesContext>>
    }
}
