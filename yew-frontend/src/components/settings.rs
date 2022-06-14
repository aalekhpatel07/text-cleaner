
use yew::{prelude::*, context::ContextHandle};
use hashbrown::{HashMap};

use crate::contexts::{ProcessorConfigNamesContext, ProcessorConfigNames};


#[derive(Debug, Clone)]
pub enum Msg {
    ProcessorConfigNamesContextUpdated(ProcessorConfigNamesContext),
    Refresh(String)
}

pub struct Settings {
    check_boxes: HashMap<String, NodeRef>,
    check_boxes_ordered: Vec<(String, NodeRef)>,
    config: ProcessorConfigNamesContext,
    _config_handle: ContextHandle<ProcessorConfigNamesContext>
}


#[derive(PartialEq, Properties)]
pub struct SettingsProp {
    #[prop_or(ProcessorConfigNames::all())]
    pub all_config: ProcessorConfigNames,
}

pub fn snake_case_to_camel_case(s: &str) -> String {
    
    let capitalized =
    s
    .replace('_', r" ")
    .split_ascii_whitespace() 
    .map(|word| {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        let rest = chars.collect::<String>();
        first_char.to_uppercase().to_string() + rest.as_str()
    })
    .collect::<Vec<String>>();

    capitalized.join(" ")
}

impl Component for Settings {
    type Message = Msg;
    type Properties = SettingsProp;

    fn create(ctx: &Context<Self>) -> Self {

        let (config_names, config_names_handle) = 
        ctx
        .link()
        .context(ctx.link().callback(Msg::ProcessorConfigNamesContextUpdated))
        .expect("Failed to link context");

        let all_config = &ctx.props().all_config;
        
        let mut check_boxes = HashMap::new();

        for function in all_config.functions.iter() {
            check_boxes.insert(function.clone(), NodeRef::default());
        }

        let mut check_boxes_ordered = all_config.functions.iter().map(|function| {
            (function.clone(), check_boxes.get(function).unwrap().clone())
        }).collect::<Vec<(String, NodeRef)>>();
        check_boxes_ordered.sort_by(|a, b| a.0.cmp(&b.0));


        Self {
            check_boxes,
            check_boxes_ordered,
            config: config_names,
            _config_handle: config_names_handle
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        // let on_value_change = ctx.link().callback(|x| Msg::Toggle(x));

        let check_boxes_ordered = self.check_boxes_ordered.clone();

        html! {
            <div class="h-[90%] w-full space-y-8">
                <h1 class="text-2xl">{"Settings"}</h1>
                <form id="settings" class="h-[90%] px-4 space-y-2 overflow-auto">
                    { for check_boxes_ordered.into_iter().map(|(key, value)| {

                        let key_cp = key.clone();
                        let on_value_change = ctx.link().callback(Msg::Refresh);

                        html! {
                            <div class="flex items-center justify-start">
                                <input 
                                    name={key.clone()} 
                                    type="checkbox"
                                    ref={value.clone()}
                                    checked={self.config.functions.contains(&key)}
                                    class="checkbox checkbox-secondary rounded-md"
                                    onchange={move |_| on_value_change.emit(key_cp.clone())}
                                />
                                <label class="label italic" for={key.clone()}>{snake_case_to_camel_case(&key)}</label>
                            </div>
                        }
                    })}
                </form>
            </div>
        }
    }


    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ProcessorConfigNamesContextUpdated(processor_config_names) => {
                self.config = processor_config_names;
            },
            Msg::Refresh(key) => {
                self.config.dispatch(key);
            }
        }
        true
    }
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        for (function_name, check_box_node) in self.check_boxes.iter() {

            if let Some(check_box) = check_box_node.cast::<web_sys::HtmlInputElement>() {

                if self.config.functions.contains(function_name) {
                    check_box.set_attribute("checked", "true").unwrap();
                } else {
                    check_box.set_attribute("checked", "false").unwrap();
                }
            }

        }
    }

}