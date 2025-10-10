use leptos::{
    html::{div, img},
    prelude::*,
};

#[component]
pub fn Page() -> impl IntoView {
    div()
        .class("rounded bg-orange-100 p-5 m-5 flex gap-5 shadow-md")
        .child((
            img()
                .src("./static/images/profile.png")
                .class("rounded shadow-md"),
            div().class("flex-col flex w-full rounded bg-yellow-100 shadow-md"),
        ))
}
