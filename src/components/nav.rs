use leptos::{
    ev,
    html::{button, div, img, nav, progress},
    prelude::*,
};
use leptos_darkmode::Darkmode;
use leptos_icons::Icon;

#[component]
pub fn Nav() -> impl IntoView {
    let mut darkmode = expect_context::<Darkmode>();

    let (scrollprogress, set_scrollprogress) = signal(0.0);
    window_event_listener(ev::scroll, move |_| {
        let window = window();
        let document = window.document().unwrap();
        let element = document.document_element().unwrap();

        let scroll_top = element.scroll_top() as f64;
        let scroll_height = element.scroll_height() as f64;
        let client_height = element.client_height() as f64;

        set_scrollprogress.set((scroll_top / (scroll_height - client_height)) * 100.0);
    });

    nav()
        .child((
            div()
                .child((
                    img().src("./static/images/profile.png").class("size-14 p-50"),
                    button()
                        .child(
                            Icon(leptos_icons::IconProps {
                                icon: Signal::derive({
                                    let darkmode_clone = darkmode.clone();
                                    move || {
                                        if darkmode_clone.is_dark() {
                                            icondata::LuMoonStar
                                        } else {
                                            icondata::LuSun
                                        }
                                    }
                                }),
                                style: MaybeProp::default(),
                                width: MaybeProp::default(),
                                height: MaybeProp::default(),
                            })
                            .attr("class", {
                                let darkmode_clone = darkmode.clone();
                                move || {
                                    darkmode_clone
                                        .is_dark()
                                        .then_some("text-foreground-dark")
                                        .unwrap_or("text-foreground")
                                }
                            }),
                        )
                        .on(ev::click, move |_| darkmode.toggle())
                        .class("h-full aspect-square"),
                ))
                .class("flex justify-between items-center"),
            progress()
                // default styling
                .class("
                    [&::-moz-progress-bar]:rounded-full absolute bottom-0 left-0 w-full h-0.5
                    [&::-moz-progress-bar]:bg-gradient-to-r [&::-moz-progress-bar]:from-terminal-azure [&::-moz-progress-bar]:to-terminal-purple bg-background-ui
                    dark:[&::-moz-progress-bar]:bg-gradient-to-r dark:[&::-moz-progress-bar]:from-terminal-azure dark:[&::-moz-progress-bar]:to-terminal-purple dark:bg-background-ui-dark"
                )
                .max(100)
                .value(move || scrollprogress.get()),
        ))
        .class(
            "shadow-md fixed w-full h-12 bg-background-ui dark:bg-background-ui-dark flex flex-col",
        )
}
