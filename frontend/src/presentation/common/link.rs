use crate::routes::RootRoutes;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    #[prop_or(String::from(""))]
    pub class: String,
    #[prop_or_default]
    pub href: Option<RootRoutes>,
    #[prop_or(String::from(""))]
    pub out_href: String,
    pub children: Children,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    let onclick_callback = match props.onclick.clone() {
        Some(callback) => callback,
        None => {
            let history = use_history().unwrap();
            let history = history.clone();
            let target = props.href.clone();
            let out_href = props.out_href.clone();

            match target {
                Some(target) => Callback::from(move |_| {
                    history.push(target.clone());
                }),
                None => Callback::from(move |_| {
                    web_sys::window()
                        .unwrap()
                        .location()
                        .set_href(out_href.as_str())
                        .unwrap();
                }),
            }
        }
    };

    html! {
        <a class={props.class.clone()} onclick={onclick_callback}>
            {props.children.clone()}
        </a>
    }
}
