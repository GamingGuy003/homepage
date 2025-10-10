use leptos::{
    html::{div, p},
    prelude::*,
};

#[component]
pub fn Id() -> impl IntoView {
    div()
        .class("rounded bg-orange-100 p-5 m-5 flex gap-5 shadow-md")
        .child((div()
            .class("flex-col flex w-full rounded bg-yellow-100 shadow-md p-5")
            .child((
                p().child("Title").class("text-lg"),
                p().child("Subtext").class("text-sm"),
            )),))
}
