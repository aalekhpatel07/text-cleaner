use log::info;
use yew::{prelude::*, context::ContextHandle};
use hashbrown::{HashMap, HashSet};

use crate::contexts::{ProcessorConfigNamesContext, TextProcessor, ProcessorConfigNames};


#[derive(Debug, Clone)]
pub enum Msg {
    ProcessorConfigNamesContextUpdated(ProcessorConfigNamesContext),
    Refresh(String)
}

pub struct Settings {
    check_boxes: HashMap<String, NodeRef>,
    config: ProcessorConfigNamesContext,
    _config_handle: ContextHandle<ProcessorConfigNamesContext>
}


#[derive(PartialEq, Properties)]
pub struct SettingsProp {
    #[prop_or(ProcessorConfigNames::all())]
    pub all_config: ProcessorConfigNames,
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

        Self {
            check_boxes,
            config: config_names,
            _config_handle: config_names_handle
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        // let on_value_change = ctx.link().callback(|x| Msg::Toggle(x));

        let check_boxes = self.check_boxes.clone();

        html! {
            <div class="h-[90%] w-full overflow-auto">
                <form id="settings" class="grid grid-cols-3">
                    { for check_boxes.into_iter().map(|(key, value)| {

                        let key_cp = key.clone();
                        let on_value_change = ctx.link().callback(|x| Msg::Refresh(x));

                        html! {
                            <div class="flex items-center justify-start">
                                <input 
                                    name={key.clone()} 
                                    type="checkbox"
                                    ref={value.clone()}
                                    checked={self.config.functions.contains(&key)}
                                    class="checkbox checkbox-secondary"
                                    onchange={move |_| on_value_change.emit(key_cp.clone())}
                                />
                                <label class="label" for={key.clone()}>{key}</label>
                            </div>
                        }
                    })}
                </form>
                {format!("{:?}", self.config.functions)}
            </div>
        }
    }


    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ProcessorConfigNamesContextUpdated(processor_config_names) => {
                info!("ProcessorConfigNamesContextUpdated {:?}", processor_config_names);
                self.config = processor_config_names;

                // for (function_name, check_box_node) in self.check_boxes.iter() {

                //     if let Some(check_box) = check_box_node.cast::<web_sys::HtmlInputElement>() {

                //         if self.config.functions.contains(function_name) {
                //             check_box.set_attribute("checked", "true").unwrap();
                //         } else {
                //             check_box.set_attribute("checked", "false").unwrap();
                //         }
                //     }
                // }
            },
            Msg::Refresh(key) => {
                self.config.dispatch(key);
            }
        }
        true
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
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