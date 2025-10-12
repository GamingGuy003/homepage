use leptos::{
    ev,
    html::{a, div, footer, p},
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
                .class("shadow-md bg-background-content dark:bg-background-content-dark h-300"),
            // footer
            footer()
                .class("bg-background-ui dark:bg-background-ui-dark text-foreground dark:text-foreground-dark h-20 place-self-center")
                .child((
                    p()
                        .class("text-foreground dark:text-foreground-dark")
                        .child(format!(
                            "commit {} built on {}",
                            env!("GIT_HASH"),
                            env!("DATE")
                        )),
                    p()
                        .child((
                            "Oneko by ",
                            a()
                                .href("https://sleepie.dev/oneko")
                                .child("sleepie.dev")
                                .target("_blank")
                            ))
                ))
        ))
        .on(ev::click, |data| leptos::logging::log!("{:?}", data))
        // darkmode toggle
        .class(move || darkmode.is_dark().then_some("dark"))
}
