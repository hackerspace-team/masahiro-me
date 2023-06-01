use crate::pages::posts::shared::{loading_post::LoadingPost, loading_posts::LoadingPosts};
use crate::pages::{
    about::index::AboutIndex,
    not_found::NotFound,
    posts::{detail::PostDetail, index::PostIndex},
    projects::index::Projects,
    shared::layout::Layout,
};
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/pages/:page")]
    PostIndex { page: i32 },
    #[at("/posts/:slug")]
    PostDetail { slug: String },
    #[at("/projects")]
    Projects,
    #[at("/about")]
    AboutIndex,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::PostIndex { page } => {
            let fallback = html! {<LoadingPosts />};
            html! {
                <Suspense {fallback}>
                    <PostIndex page={page.clone()} />
                </Suspense>
            }
        }
        Route::PostDetail { slug } => {
            let fallback = html! {<LoadingPost />};
            html! {
                <Suspense {fallback}>
                    <PostDetail slug={slug.clone()} />
                </Suspense>
            }
        }
        Route::Projects => html! { <Projects /> },
        Route::AboutIndex => html! { <AboutIndex /> },
        Route::NotFound => html! { <NotFound />},
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
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </Router>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}
