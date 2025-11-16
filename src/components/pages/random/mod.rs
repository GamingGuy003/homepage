use std::sync::Arc;

use leptos::{
    html::{a, div, li, p, ul},
    prelude::*,
};

use crate::components::pages::about::card::{Card, CardProps};

#[component]
pub fn Random() -> impl IntoView {
    div().class("grid gap-5 place-items-center").child((
            Card(
                CardProps::builder()
                    .hover(false)
                    .class(Some("md:col-span-3 w-full"))
                    .title("Random")
                    .children(Arc::new(|| {
                        p().class("text-center")
                            .child("This somewhat hidden section contains random things I add whenever I feel like it")
                            .into_any()
                    }))
                    .build(),
            ),
            Card(
                CardProps::builder()
                    .hover(false)
                    .class(Some("col-span-1 w-full"))
                    .title("Artists")
                    .children(Arc::new(|| {
                        div()
                            .child((
                                p().child("Some of my favorite artists / music at the moment"),
                                ul()
                                    .class("list-disc pl-8 italic")    
                                    .child((
                                        li().child(a().child("Elita - Girls on the Internet").href("https://youtu.be/K5CmL-R_D4U").target("_blank")),
                                        li().child(a().child("Shotgun Willy - Dalai Lama").href("https://youtu.be/r3npD8aCS9E").target("_blank"))
                                    ))
                            ))
                            .into_any()
                    })).build()
            )
    ))
}
