use leptos::{
    ev,
    html::{div, footer},
    prelude::*,
};
use leptos_darkmode::Darkmode;
use leptos_meta::provide_meta_context;

use crate::components::nav::Nav;

#[component]
pub fn Page() -> impl IntoView {
    provide_meta_context();
    let darkmode = Darkmode::init();

    div()
        .child((
            // navbar
            Nav(),
            // content
            div()
                .style("height: 200vh")
                .class("shadow-md bg-background-content dark:bg-background-content-dark"),
            // footer
            footer()
                .child(format!(
                    "commit {} built on {}",
                    env!("GIT_HASH"),
                    env!("DATE")
                ))
                .class("bg-background-ui dark:bg-background-ui-dark text-foreground dark:text-foreground-dark h-20 place-self-center"),
        ))
        .on(ev::click, |data| leptos::logging::log!("{:?}", data))
        // darkmode toggle
        .class(move || darkmode.is_dark().then(|| Some("dark")))
}
