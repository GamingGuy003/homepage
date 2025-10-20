use leptos::{
    html::{div, iframe},
    prelude::*,
};
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
    div().class("h-full bg-red").child(
        iframe()
            .class("w-full h-full rounded-xl shadow-xl")
            .src(based_url(&format!("./static/projects/{}", file()))),
    )
}
