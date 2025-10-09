use leptos::{
    html::{div, h1, img, li, nav, ol, p},
    prelude::*,
};

#[component]
pub fn Page() -> impl IntoView {
    div()
        .class("rounded bg-orange-100 border")
        .child(img().src("/static/images/profile.png"))
}
