use leptos::{
    ev,
    html::{div, footer, p},
    prelude::*,
};
use leptos_darkmode::Darkmode;
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{Route, RouteChildren, RouteProps, Routes, RoutesProps},
    path,
};

use crate::components::{
    nav::Nav,
    pages::{about::About, landing::Landing, projects::p1::Project1},
};

#[component]
pub fn Page() -> impl IntoView {
    provide_meta_context();
    let darkmode = Darkmode::init();
    let props = RoutesProps {
        fallback: || "Route not found",
        transition: true,
        children: RouteChildren::to_children(|| {
            (
                Route(RouteProps {
                    path: path!("/"),
                    view: Landing,
                    ssr: leptos_router::SsrMode::OutOfOrder,
                }),
                Route(RouteProps {
                    path: path!("/about"),
                    view: About,
                    ssr: leptos_router::SsrMode::OutOfOrder,
                }),
                Route(RouteProps {
                    path: path!("/p1"),
                    view: Project1,
                    ssr: leptos_router::SsrMode::OutOfOrder,
                }),
            )
        }),
    };

    div()
        .child((
            // navbar
            Nav(),
            // content
            div()
                .class("pt-16 px-5 min-h-screen bg-background-content dark:bg-background-content-dark text-foreground dark:text-foreground-dark")
                .child(Routes(props)),
            // footer
            footer()
                .class("bg-background-ui dark:bg-background-ui-dark text-foreground dark:text-foreground-dark py-5 text-center")
                .child((
                    p()
                        .child(format!(
                            "Commit {} built on {} by",
                            env!("GIT_HASH"),
                            env!("DATE"),
                        )),
                        p().child(env!("UNAME"))
                ))
        ))
        .on(ev::click, |data| leptos::logging::log!("{:?}", data))
        // darkmode toggle
        .class(move || darkmode.is_dark().then_some("dark"))
}
