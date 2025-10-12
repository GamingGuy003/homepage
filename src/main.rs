mod components;

use components::page::Page;
use leptos::prelude::*;
use leptos_router::components::{Router, RouterProps};

fn main() {
    console_error_panic_hook::set_once();

    let children = TypedChildren::to_children(Page);
    let props = RouterProps::builder().children(children).build();
    leptos::mount::mount_to_body(|| Router(props));
}
