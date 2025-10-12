use leptos::{
    html::{div, h1},
    prelude::*,
};

#[component]
pub fn About() -> impl IntoView {
    div()
        .class("grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2")
        .child(
            div()
                .class("
                    col-span-3 bg-background-ui dark:bg-background-ui-dark rounded border-1 text-center text-foreground dark:text-foreground-dark
                    shadow-md transition-transform duration-100 hover:scale-102 border-gradient-to-r from-terminal-purple to-terminal-azure")
                .child(h1().child("About me")),
        )
}
