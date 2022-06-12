use yew::prelude::*;

use crate::contexts::Theme;


#[function_component(Textbox)]
pub fn app() -> Html {
    let input = use_state(|| "".to_string());
    let theme = use_context::<Theme>().expect("No Theme context found.");

    let placeholder = "Enter some text here.....".to_owned();
    let input_str = (*input).clone();

    html! {
        <div>
            <label for={"inputBox"}>
                {"Text Cleaner"}
            </label>
            <textarea 
                name={"inputBox"} 
                rows="40" 
                cols="auto"
                placeholder={placeholder}
            >
            </textarea>
            {input_str}
            {theme}
        </div>
    }
}