use leptos::prelude::*;

use leptos::html::{div, p};

#[component]
pub fn Card<'a>(title: &'a str, children: ChildrenFn, class: Option<&'a str>) -> impl IntoView {
    div()
        .class(format!("
            rounded-md text-center h-full dark:bg-background-ui-dark
            shadow-xl transition-transform duration-100 hover:scale-102
            hover:bg-gradient-to-br
            from-terminal-yellow-dark dark:from-terminal-purple-dark
            via-terminal-orange dark:via-none
            to-terminal-red-dark dark:to-terminal-azure-dark
            p-0.75 {}", class.unwrap_or_default()
        ))
        .child(
            div().class("h-full flex flex-col").child((
                p().class(
                    "
                        text-foreground dark:text-foreground-dark bg-background-content
                        dark:bg-background-ui-dark rounded-t-sm shadow-md p-3 font-semibold flex-none
                    ",
                )
                .child(title),
                div()
                    .class(
                        "
                        bg-background-ui dark:bg-background-content-dark text-foreground
                        dark:text-foreground-dark rounded-b-sm p-3 flex-1 text-justify shadow-md
                    ",
                    )
                    .child(children()),
            )),
        )
}
