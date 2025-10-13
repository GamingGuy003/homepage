use leptos::prelude::*;

use leptos::html::{div, p};

#[component]
pub fn Languages() -> impl IntoView {
    div()
        .child((
            p().child(
                "Over the years I have used quite a few programming languages,
                here are some I am the most proficient with / like the most"
            ),
            div()
                .class("p-5 relative")
                .child((
                    div()
                        .class(
                            "bg-gradient-to-t from-terminal-red
                            via-terminal-yellow-dark to-terminal-green-dark relative h-[min(40vh,40vw)] w-1
                        ")
                        .child(div()
                            .class("absolute left-0 top-1/2 -translate-y-1/2 -translate-x-1/6 -rotate-90 origin-left whitespace-nowrap")
                            .child("Enjoyment")),
                    div()
                        .class(
                            "bg-gradient-to-r from-terminal-red via-terminal-yellow-dark
                            to-terminal-green-dark h-1 w-full text-center"
                        )
                        .child("Knowledge"),
                    div()
                        .class("absolute bottom-[90%] left-[85%] -translate-x-1/2")
                        .child("Rust")
                        .title("Fun if you like fighting your compiler"),
                    div()
                        .class("absolute bottom-[30%] left-[65%] -translate-x-1/2")
                        .child("JS")
                        .title("Dynamic types will be humanities downfall"),
                    div()
                        .class("absolute bottom-[50%] left-[70%] -translate-x-1/2")
                        .child("TS")
                        .title("Can't be worse than JS"),
                    div()
                        .class("absolute bottom-[55%] left-[60%] -translate-x-1/2")
                        .child("C")
                        .title("Fun but requires 200IQ"),
                    div()
                        .class("absolute bottom-[40%] left-[50%] -translate-x-1/2")
                        .child("CPP")
                        .title("Bloated"),
                    div()
                        .class("absolute bottom-[60%] left-[35%] -translate-x-1/2")
                        .child("Python")
                        .title("Rarely used but fun if typed"),
                    div()
                        .class("absolute bottom-[60%] left-[75%] -translate-x-1/2")
                        .child("Java")
                        .title("Used lots, could be worse"),
                    div()
                        .class("absolute bottom-[80%] left-[20%] -translate-x-1/2")
                        .child("Lua")
                        .title("Rarely used"),
                ))
        ))
}
