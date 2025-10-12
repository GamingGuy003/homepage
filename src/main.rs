mod components;

use components::page::Page;
use leptos::prelude::*;
use leptos_router::components::{Router, RouterProps};

fn main() {
    console_error_panic_hook::set_once();

    let children = TypedChildren::to_children(Page);
    let props = RouterProps::builder()
        .children(children)
        .base(based_url(""))
        .build();

    leptos::mount::mount_to_body(|| Router(props));
}

// adds the appropriate base to a url
pub fn based_url(url: &str) -> String {
    // add slash to start of uri if len > 0
    let slashed = if url.len() > 0 && !url.starts_with('/') {
        format!("/{url}")
    } else {
        url.to_owned()
    };
    format!(
        "{}{}",
        option_env!("BASE_PATH").unwrap_or_default(),
        slashed
    )
}
