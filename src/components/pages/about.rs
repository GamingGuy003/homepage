use std::sync::Arc;

use leptos::{
    html::{a, button, div, img, p},
    prelude::*,
};
use leptos_icons::Icon;

#[component]
pub fn About() -> impl IntoView {
    div()
        .class("grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3")
        .child((
            Card(
                CardProps::builder()
                    .class(Some("col-span-1 md:col-span-2 row-span-2"))
                    .title("About me")
                    .children(Arc::new(|| {
                        div()
                            .child((
                                p().child("My name is Felix,"),
                                div().class("flex flex-col md:flex-row gap-5 pb-5").child((
                                    // text column
                                    div()
                                        .class("flex-1")
                                        .child((
                                            p().child(
                                                "I currently study business informatics.
                                                My main interest is backend development, which is why
                                                I've kept this page pretty minimalistic. I enjoy building
                                                and modifying keyboards although it is a pretty expensive
                                                hobby sadly. In addition to my slight keyboard addiction,
                                                I am also an avid linux user, having used most distros for
                                                a while.",
                                            ),
                                        )),
                                    Images(),
                                )),
                                p().child(
                                    "Also don't forget to say hi to dinguin, my little mascott!",
                                ),
                            ))
                            .into_any()
                    }))
                    .build(),
            ),
            Card(
                CardProps::builder()
                    .class(Some("col-span-1"))
                    .title("This page")
                    .children(Arc::new(|| {
                        div()
                            .child(p().child(
                                "This page has been written almost entirely using rust
                                webassembly. This was done mostly for myself to have an excuse
                                to waste some hours learning it, but it also comes with
                                the upside of this page being built entirely without javascript.
                                ",
                            ))
                            .into_any()
                    }))
                    .build(),
            ),
            Card(
                CardProps::builder()
                    .class(Some("col-span-1"))
                    .title("Contact")
                    .children(Arc::new(|| {
                        div()
                            .child((
                                p().child(
                                    "I am not active on any social media, however feel free
                                    to look into my doings or reach out to me under any of the following:",
                                ),
                                div()
                                    .class("grid grid-cols-2 grid-rows-2 gap-3 py-3")
                                    .child((
                                        button()
                                            .class(
                                                "rounded-md bg-background-ui dark:bg-background-ui-dark p-1 shadow-md flex
                                                transition-transform duration-100 hover:scale-95 active:scale-75 px-5"
                                            )
                                            .child((
                                                Icon(leptos_icons::IconProps {
                                                    icon: Signal::derive(|| icondata::LuGithub),
                                                    style: MaybeProp::default(),
                                                    width: MaybeProp::default(),
                                                    height: MaybeProp::default(),
                                                })
                                                .attr("class", "h-full"),
                                                a()
                                                    .class("w-full")
                                                    .href("https://github.com/GamingGuy003")
                                                    .target("_blank")
                                                    .child("Github"),
                                            )),
                                        button()
                                            .class(
                                                "rounded-md bg-background-ui dark:bg-background-ui-dark p-1 shadow-md flex
                                                transition-transform duration-100 hover:scale-95 active:scale-75 px-5"
                                            )
                                            .child((
                                                Icon(leptos_icons::IconProps {
                                                    icon: Signal::derive(|| icondata::LuMail),
                                                    style: MaybeProp::default(),
                                                    width: MaybeProp::default(),
                                                    height: MaybeProp::default(),
                                                })
                                                .attr("class", "h-full"),
                                                a()
                                                    .class("w-full")
                                                    .href("mailto:bjhell@unibz.it")
                                                    .target("_blank")
                                                    .child("Email")
                                            )),
                                        button()
                                            .class(
                                                "rounded-md bg-background-ui dark:bg-background-ui-dark p-1 shadow-md flex
                                                transition-transform duration-100 hover:scale-95 active:scale-75 px-5"
                                            )
                                            .child((
                                                Icon(leptos_icons::IconProps {
                                                    icon: Signal::derive(|| icondata::ImSpotify),
                                                    style: MaybeProp::default(),
                                                    width: MaybeProp::default(),
                                                    height: MaybeProp::default(),
                                                })
                                                .attr("class", "h-full"),
                                                a()
                                                    .class("w-full")
                                                    .href("https://open.spotify.com/user/5rh4hrchdlrl1uhmc3lqqcia1")
                                                    .target("_blank")
                                                    .child("Spotify")
                                            ))
                                    )),
                                p().class("text-xs").child("I will do my best to respond within 24 hours!"),
                            ))
                            .into_any()
                    }))
                    .build(),
            ),
            Card(CardProps::builder()
                .class(Some("col-span-1 md:col-span-2 lg:col-span-3"))
                .title("Languages")
                .children(Arc::new(|| Languages().into_any()))
                .build()
            )
        ))
}

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
                            "bg-gradient-to-t from-terminal-red dark:from-terminal-green-red
                            via-terminal-yellow dark:via-terminal-yellow-dark relative
                            to-terminal-green dark:to-terminal-green-dark h-[50vh] w-1
                        ")
                        .child(div()
                            .class("absolute left-0 top-1/2 -translate-y-1/2 -translate-x-1/6 -rotate-90 origin-left whitespace-nowrap")
                            .child("Enjoyment")),
                    div()
                        .class(
                            "bg-gradient-to-r from-terminal-red dark:from-terminal-green-red
                            via-terminal-yellow dark:via-terminal-yellow-dark
                            to-terminal-green dark:to-terminal-green-dark h-1 w-full
                            text-center
                        ")
                        .child("Knowledge"),
                    div()
                        .class("absolute bottom-[90%] left-[85%] -translate-x-1/2")
                        .child("Rust"),
                    div()
                        .class("absolute bottom-[30%] left-[65%] -translate-x-1/2")
                        .child("JS"),
                    div()
                        .class("absolute bottom-[50%] left-[70%] -translate-x-1/2")
                        .child("TS"),
                    div()
                        .class("absolute bottom-[55%] left-[60%] -translate-x-1/2")
                        .child("C"),
                    div()
                        .class("absolute bottom-[40%] left-[50%] -translate-x-1/2")
                        .child("CPP"),
                    div()
                        .class("absolute bottom-[60%] left-[35%] -translate-x-1/2")
                        .child("Python"),
                    div()
                        .class("absolute bottom-[60%] left-[75%] -translate-x-1/2")
                        .child("Java"),
                    div()
                        .class("absolute bottom-[80%] left-[20%] -translate-x-1/2")
                        .child("Lua"),
                ))
        ))
}

#[component]
pub fn Card<'a>(title: &'a str, children: ChildrenFn, class: Option<&'a str>) -> impl IntoView {
    div()
        .class(format!("
            rounded-md text-center h-full
            shadow-xl transition-transform duration-100 hover:scale-102
            bg-gradient-to-tl hover:bg-gradient-to-br
            from-terminal-red dark:from-terminal-purple-dark
            via-terminal-orange dark:via-none
            to-terminal-yellow-dark dark:to-terminal-azure-dark
            p-0.75 {}", class.unwrap_or_default()
        ))
        .child(
            div().class("h-full flex flex-col shadow-xl").child((
                p().class(
                    "
                        text-foreground dark:text-foreground-dark bg-background-content
                        dark:bg-background-ui-dark rounded-t-sm shadow-xl p-3 font-semibold flex-none
                    ",
                )
                .child(title),
                div()
                    .class(
                        "
                        bg-background-ui dark:bg-background-content-dark text-foreground
                        dark:text-foreground-dark rounded-b-sm p-3 flex-1 text-justify
                    ",
                    )
                    .child(children()),
            )),
        )
}

#[component]
pub fn Images() -> impl IntoView {
    div()
        .class("w-full flex flex-1 justify-center max-h-[50vh]")
        .child(div()
            .class("grid grid-cols-2 grid-rows-2 gap-3 ")
            .child((
                div()
                    .class("relative overflow-hidden rounded-lg group")
                    .child((
                        img()
                            .class("h-full rounded-lg shadow-md object-cover object-center")
                            .src("./static/images/keyboard.jpg")
                            .alt("Keyboard and mouse on table"),
                        div()
                            .class(
                                "absolute inset-0 flex items-center text-center justify-center
                                text-foreground dark:text-foreground-dark text-xl font-semibold opacity-0
                                bg-background-ui dark:bg-background-ui-dark
                                group-hover:opacity-50 transition duration-250")
                            .child("Current keyboard")
                    )),
                div()
                    .class("shadow-md rounded-lg relative overflow-hidden group flex")
                    .child((
                        div()
                            .class("grid grid-cols-2 grid-rows-2 gap-2 p-2 max-h-full")
                            .child((
                                img()
                                    .class("h-full aspect-square")
                                    .src("https://upload.wikimedia.org/wikipedia/commons/thumb/3/3f/Fedora_logo.svg/2048px-Fedora_logo.svg.png")
                                    .alt("Fedora my beloved"),
                                img()
                                    .class("h-full aspect-square")
                                    .src("https://upload.wikimedia.org/wikipedia/commons/thumb/1/13/Arch_Linux_%22Crystal%22_icon.svg/2048px-Arch_Linux_%22Crystal%22_icon.svg.png")
                                    .alt("Arch my first"),
                                img()
                                    .class("h-full aspect-square")
                                    .class("h-full")
                                    .src("https://upload.wikimedia.org/wikipedia/commons/1/19/Gentoo_Logo_Vector.svg")
                                    .alt("Gentoo for maximum braindamage"),
                                img()
                                    .class("h-full aspect-square")
                                    .src("https://avatars.githubusercontent.com/u/2678585?s=200&v=4")
                                    .alt("Ol'reliable")
                            )),
                        div()
                            .class(
                                "absolute inset-0 flex items-center text-center justify-center
                                text-foreground dark:text-foreground-dark text-xl font-semibold opacity-0
                                bg-background-ui dark:bg-background-ui-dark
                                group-hover:opacity-50 transition duration-250")
                            .child("Favorite distros")

                    )),
                div()
                    .class("relative overflow-hidden rounded-lg group col-span-2")
                    .child((
                        img()
                            .class("h-full w-full shadow-md col-span-2 rounded-lg object-cover object-center")
                            .src("./static/images/germany.jpg")
                            .alt("Germany"),
                        div()
                            .class(
                                "absolute inset-0 flex items-center text-center justify-center
                                text-foreground dark:text-foreground-dark text-xl font-semibold opacity-0
                                bg-background-ui dark:bg-background-ui-dark
                                group-hover:opacity-50 transition duration-250")
                            .child("Picture taken in Germany")
                    ))
            ))
    )
}
