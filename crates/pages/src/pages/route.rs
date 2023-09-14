use super::posts::shared::{loading_post, loading_posts};
use super::{about, not_found, posts, projects};
use std::str::FromStr;
use url::Url;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
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

impl FromStr for Route {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(&format!("http://localhost:8080{}", s))?;
        let Some(path_segments) = url.path_segments() else {
            return Ok(Self::NotFound);
        };
        let path_segments = path_segments.collect::<Vec<_>>();
        if let Some(&"about") = path_segments.get(0) {
            return Ok(Self::AboutIndex {});
        }
        if let Some(&"projects") = path_segments.get(0) {
            return Ok(Self::Projects {});
        }
        if let (Some(&"pages"), Some(page)) = (path_segments.get(0), path_segments.get(1)) {
            return Ok(Self::PostIndex {
                page: page.to_string().parse().unwrap_or(1),
            });
        }
        if let (Some(&"posts"), Some(slug)) = (path_segments.get(0), path_segments.get(1)) {
            return Ok(Self::PostDetail {
                slug: slug.to_string(),
            });
        }
        if let Some(&"") = path_segments.get(0) {
            return Ok(Self::PostIndex { page: 1 });
        }
        Ok(Self::NotFound)
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::PostIndex { page } => {
            let fallback = html! {<loading_posts::LoadingPosts />};
            html! {
                <Suspense {fallback}>
                    <posts::index::PostIndex page={page.clone()} />
                </Suspense>
            }
        }
        Route::PostDetail { slug } => {
            let fallback = html! {<loading_post::LoadingPost />};
            html! {
                <Suspense {fallback}>
                    <posts::detail::PostDetail slug={slug.clone()} />
                </Suspense>
            }
        }
        Route::Projects => {
            html! { <projects::index::Projects /> }
        }
        Route::AboutIndex => {
            html! { <about::index::AboutIndex /> }
        }
        Route::NotFound => html! { <not_found::NotFound />},
    }
}
