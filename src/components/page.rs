use leptos::{
    html::{div, img, nav},
    prelude::*,
};

use crate::components::id::Id;

#[component]
pub fn Page() -> impl IntoView {
    div().child((
        nav()
            .child(img().src("./static/images/profile.png").class("size-16"))
            .class("shadow-md"),
        Id(),
    ))
}
