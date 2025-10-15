use leptos::{html::div, prelude::*};

use crate::components::pages::projects::p1::Project1;

pub mod p1;
mod project;
mod project_card;

#[component]
pub fn Projects() -> impl IntoView {
    div()
        .class("grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3")
        .child(Project1())
}
