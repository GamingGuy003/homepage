use leptos::{html::iframe, prelude::*};
use leptos_router::{hooks::use_params, params::Params};

use crate::based_url;

#[derive(Params, PartialEq)]
struct ProjectParam {
    file: Option<String>,
}

#[component]
pub fn ProjectView() -> impl IntoView {
    let params = use_params::<ProjectParam>();
    let file = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.file.clone())
            .unwrap_or_default()
    };
    iframe()
        .class("w-full rounded-xl shadow-xl flex-1 relative")
        .src(based_url(&format!("./static/projects/{}", file())))
}
