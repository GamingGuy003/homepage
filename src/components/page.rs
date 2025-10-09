use leptos::{
    html::{div, img},
    prelude::*,
};

#[component]
pub fn Page() -> impl IntoView {
    div()
        .class("rounded bg-orange-100 border")
        .child(img().src("./static/images/profile.png"))
}
