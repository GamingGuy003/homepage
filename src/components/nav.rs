use leptos::{
    ev,
    html::{a, button, div, img, nav, progress},
    prelude::*,
};
use leptos_darkmode::Darkmode;
use leptos_icons::Icon;

use crate::based_url;

#[component]
pub fn Nav() -> impl IntoView {
    let mut darkmode = expect_context::<Darkmode>();

    let (scrollprogress, set_scrollprogress) = signal(0.0);
    window_event_listener(ev::scroll, move |_| {
        let document = document();

        let body = document.body().unwrap();
        let html = document.document_element().unwrap();

        let scroll_top = html.scroll_top() as f64;
        let scroll_height = body.scroll_height().max(html.scroll_height()) as f64;
        let client_height = window().inner_height().unwrap().as_f64().unwrap();

        set_scrollprogress.set((scroll_top / (scroll_height - client_height)) * 100.0);
    });

    nav()
        .class("fixed w-full top-0 left-0 h-14 z-50 bg-background-ui/50 dark:bg-background-ui-dark/50 backdrop-blur-md flex flex-col shadow-md")
        .child((
            div()
                .class("flex justify-between items-center h-full")
                .child((
                    div()
                        .class("flex items-center")
                        .child(
                            a()
                                .href(based_url("/"))
                                .child(
                                    img()
                                        .class("block max-h-10 dark:brightness-85 transition-transform duration-100 hover:scale-95 active:scale-75")
                                        .src("./static/images/profile.png")
                                        .alt("Logo")
                                )
                        ),
                    button()
                        .class(
                            "aspect-square flex align-center items-center w-12 xs:w-32
                            transition-transform duration-100 hover:scale-95 cursor-pointer
                            text-foreground dark:text-foreground-dark px-2 m-2"
                        )
                        .child(
                            Icon(leptos_icons::IconProps {
                                icon: Signal::derive({
                                    let darkmode_clone = darkmode.clone();
                                    move || if darkmode_clone.is_dark() { icondata::LuMoonStar } else { icondata::LuSun }
                                }),
                                style: "width: 100%".into(),
                                width: MaybeProp::default(),
                                height: MaybeProp::default(),
                            })
                            .attr("class", "h-full")
                        )
                        // cant move this up or things explode
                        .on(ev::click, move |_| darkmode.toggle())
                )),
            progress()
                .class("
                    [&::-moz-progress-bar]:rounded-full absolute bottom-0 left-0 w-full h-0.5
                    [&::-moz-progress-bar]:bg-gradient-to-r [&::-moz-progress-bar]:from-terminal-orange [&::-moz-progress-bar]:to-terminal-red bg-background-ui
                    dark:[&::-moz-progress-bar]:bg-gradient-to-r dark:[&::-moz-progress-bar]:from-terminal-azure-dark dark:[&::-moz-progress-bar]:to-terminal-purple-dark dark:bg-background-ui-dark"
                )
                .max(100)
                .value(move || scrollprogress.get()),
        ))
}
