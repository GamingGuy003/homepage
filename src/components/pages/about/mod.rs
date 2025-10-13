use std::sync::Arc;

use leptos::{
    html::{a, div, img, p},
    prelude::*,
};
use leptos_icons::Icon;

pub mod card;
mod images;
mod languages;

use crate::components::pages::about::{
    card::{Card, CardProps},
    images::Images,
    languages::Languages,
};

#[component]
pub fn About() -> impl IntoView {
    div()
        .class("grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5")
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
                                                a while."
                                            ),
                                            p().child(
                                                "As you can probably tell by what you've read so far, I
                                                enjoy learning new skils and testing myself and my patience.
                                                This has led me down quite a few rabbit holes, mainly IT
                                                related. I manage my own little server cluster and also 
                                                selfhost quite a few services like a cloud, password manager,
                                                streaming site and various gameservers. While I do not claim
                                                to be professional in any way, I do like to think that I know
                                                my way around the digital world"
                                            )
                                        )),
                                    Images(),
                                )),
                                p().class("text-xs text-center").child(
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
                            .child(
                                p().child((
                                    "This page has been written almost entirely using rust
                                    webassembly. This was done mostly for myself to have an excuse
                                    to waste some hours learning it, but it also comes with
                                    the upside of this page being built entirely without javascript
                                    which should bring joy to the fellow JS haters out there. In addition
                                    to that, the page also gets automatically redeployed whenever any
                                    changes are made. Thus the build timestamp at the bottom of the page
                                    will always be the time of the last page update. If you are interested
                                    in the way this page is built / deployed, go look at the ",
                                    a()
                                        .href("https://github.com/GamingGuy003/homepage")
                                        .target("_blank")
                                        .child("Github Repo"),
                                )),
                            )
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
                                        a()
                                            .class(
                                                "flex flex-row justify-center gap-3 rounded-md bg-background-content dark:bg-background-ui-dark
                                                py-2 shadow-md transition-transform duration-100 hover:scale-95"
                                            )
                                            .href("https://github.com/GamingGuy003")
                                            .target("_blank")
                                            .child((
                                                Icon(leptos_icons::IconProps {
                                                    icon: Signal::derive(|| icondata::LuGithub),
                                                    style: MaybeProp::default(),
                                                    width: MaybeProp::default(),
                                                    height: MaybeProp::default(),
                                                })
                                                .attr("class", "h-full"),
                                                "Github"
                                            )),
                                        a()
                                            .class(
                                                "flex flex-row justify-center gap-3 rounded-md bg-background-content dark:bg-background-ui-dark
                                                py-2 shadow-md transition-transform duration-100 hover:scale-95"
                                            )
                                            .href("mailto:bjhell@unibz.it")
                                            .target("_blank")
                                            .child((
                                                Icon(leptos_icons::IconProps {
                                                    icon: Signal::derive(|| icondata::LuMail),
                                                    style: MaybeProp::default(),
                                                    width: MaybeProp::default(),
                                                    height: MaybeProp::default(),
                                                })
                                                .attr("class", "h-full"),
                                                "Email"
                                            )),
                                        a()
                                            .class(
                                                "flex flex-row justify-center gap-3 rounded-md bg-background-content dark:bg-background-ui-dark
                                                py-2 shadow-md transition-transform duration-100 hover:scale-95"
                                            )
                                            .href("https://open.spotify.com/user/5rh4hrchdlrl1uhmc3lqqcia1")
                                            .target("_blank")
                                            .child((
                                                Icon(leptos_icons::IconProps {
                                                    icon: Signal::derive(|| icondata::ImSpotify),
                                                    style: MaybeProp::default(),
                                                    width: MaybeProp::default(),
                                                    height: MaybeProp::default(),
                                                })
                                                .attr("class", "h-full"),
                                                "Spotify"
                                            )),
                                    )),
                                p().class("text-xs").child("I will do my best to respond within 24 hours!"),
                            ))
                            .into_any()
                    }))
                    .build(),
            ),
Card(CardProps::builder()
                .class(Some("col-span-1 md:col-span-2 lg:col-span-1"))
                .title("About dinguin")
                .children(Arc::new(|| div()
                        .class("flex flex-row lg:flex-col align-items-center")
                        .child((
                            p().class("w-full").child(
                                "This is Dinguin, my mascott. It was an attempt I made a while ago at
                                drawing the Linux Tux. Since then it has been many places, including my
                                Github, which it has declared its permanent home. Now its domain is expanding
                                onto this website. Be nice to him!"
                            ),
                            img()
                                .class("w-[50vw] h-full aspect-square object-cover")
                                .src("./static/images/profile.png")
                                .alt("Dinguin")
                    )).into_any()))
                .build()
            ),
            Card(CardProps::builder()
                .class(Some("col-span-1 md:col-span-2"))
                .title("Languages")
                .children(Arc::new(|| Languages().into_any()))
                .build()
            ),
            Card(CardProps::builder()
                .class(None)
                .title("Privacy")
                .children(Arc::new(||
                    "In addition to the site being open source, I collect no data whatsover.
                    The only information being stored is your theme preference in your browser".into_any()
                ))
                .build()
            ),
            Card(CardProps::builder()
                .class(None)
                .title("Cookies")
                .children(Arc::new(||
                    "This website does not use cookies in any form".into_any()
                ))
                .build()
            ),
            Card(CardProps::builder()
                .class(None)
                .title("Copyright")
                .children(Arc::new(||
                    "This website and its contents are my intellectual property".into_any()
                ))
                .build()
            )
        ))
}
