use std::sync::Arc;

use leptos::{
    html::{a, div, p},
    prelude::*,
};
use leptos_icons::{Icon, IconProps};

use crate::{
    based_url,
    components::pages::about::card::{Card, CardProps},
};

#[component]
pub fn Landing() -> impl IntoView {
    div()
        .class("mt-[min(15vw,15vh)] grid gap-3 max-h-screen place-items-center")
        .child((
            div()
                .class(
                    "col-span-1 md:col-span-2 justify-center flex bg-gradient-to-br
                    from-terminal-red-dark dark:from-terminal-purple
                    via-terminal-yellow-dark dark:via-terminal-azure
                    to-terminal-green-dark dark:to-background-ui-dark
                    bg-[length:150%_150%] rounded-xl shadow-xl animate-[gradient_3s_linear_infinite]"
                )
                .child(
                    Icon(IconProps::builder().icon(icondata::LuTerminal).build())
                        .attr("class", "text-foreground dark:text-foreground-dark h-[min(30vh, 50vw)] w-full -my-4 object-cover"),
                ),
            Card(
                CardProps::builder()
                    .class(Some("col-span-1 md:col-span-2 w-full"))
                    .title("Welcome")
                    .children(Arc::new(|| {
                        p().class("text-center")
                            .child("Welcome to my homepage! Feel free to have a look around")
                            .into_any()
                    }))
                    .build(),
            ),
            a().class("w-full h-full")
                .href(based_url("/about"))
                .child(Card(
                    CardProps::builder()
                        .class(None)
                        .title("About me")
                        .children(Arc::new(|| {
                            "Some information about me and this website".into_any()
                        }))
                        .build(),
                )),
            a().class("w-full h-full")
                .href(based_url("/projects"))
                .child(Card(
                    CardProps::builder()
                        .class(None)
                        .title("Projects")
                        .children(Arc::new(|| {
                            "Projects I have worked or am still working on".into_any()
                        }))
                        .build(),
                )),
        ))
}
