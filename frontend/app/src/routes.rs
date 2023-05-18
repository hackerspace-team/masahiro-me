use crate::pages::pages::Pages;
use crate::pages::{about::About, not_found::NotFound, post::PostDetail, projects::Projects};
use crate::presentation::layout::BaseLayout;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

#[macro_export]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum RootRoutes {
    #[at("/pages/:page")]
    Pages { page: i32 },
    #[at("/posts/:slug")]
    PostDetail { slug: String },
    #[at("/projects")]
    Projects,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: RootRoutes) -> Html {
    match routes {
        RootRoutes::Pages { page } => html! {<Pages page={page.clone()} />},
        RootRoutes::PostDetail { slug } => html! {<PostDetail slug={slug.clone()} />},
        RootRoutes::Projects => html! { <Projects /> },
        RootRoutes::About => html! { <About /> },
        RootRoutes::NotFound => html! { <NotFound />},
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Switch<RootRoutes> render={switch} />
        </Router>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <BaseLayout>
                <Switch<RootRoutes> render={switch} />
            </BaseLayout>
        </BrowserRouter>
    }
}
