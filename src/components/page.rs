use leptos::{
    ev,
    html::{div, footer, img, nav, progress},
    prelude::*,
};

#[component]
pub fn Page() -> impl IntoView {
    let (scrollprogress, set_scrollprogress) = signal(0.0);

    window_event_listener(ev::scroll, move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.document_element().unwrap();

        let scroll_top = element.scroll_top() as f64;
        let scroll_height = element.scroll_height() as f64;
        let client_height = element.client_height() as f64;

        set_scrollprogress.set((scroll_top / (scroll_height - client_height)) * 100.0);
    });

    div()
        .child((
            nav()
                .child(
                    div().class("flex-col flex").child((
                        img().src("./static/images/profile.png").class("size-16"),
                        progress()
                            .class("h-0.5")
                            .max(100)
                            .value(move || scrollprogress.get()),
                    )),
                )
                .class("shadow-md fixed w-full"),
            div().style("height: 200vh").class("shadow-md"),
            footer().child(format!(
                "Commit {} built on {}",
                env!("GIT_HASH"),
                env!("DATE")
            )),
        ))
        .on(ev::click, |data| leptos::logging::log!("{:?}", data))
}
