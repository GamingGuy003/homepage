use leptos::{
    html::{a, div, footer, p},
    prelude::*,
};
use leptos_darkmode::Darkmode;
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{Route, RouteChildren, RouteProps, Routes, RoutesProps},
    path,
};

use crate::{
    based_url,
    components::{
        nav::Nav,
        pages::{about::About, landing::Landing, projects::p1::Project1},
    },
};

#[component]
pub fn Page() -> impl IntoView {
    provide_meta_context();
    let darkmode = Darkmode::init();
    let props = RoutesProps {
        fallback: || "Route not found. Tough luck",
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
                .class("p-5 pt-19 min-h-screen bg-background-content dark:bg-background-content-dark text-foreground dark:text-foreground-dark")
                .child(Routes(props)),
            // footer
            footer()
                .class("bg-background-ui dark:bg-background-ui-dark text-foreground dark:text-foreground-dark py-5 text-center")
                .child((
                    a().href(based_url("/about")).child("About me"),
                    p()
                        .child(format!(
                            "v{} on Commit {} built on {} by",
                            env!("CARGO_PKG_VERSION"),
                            env!("GIT_HASH"),
                            env!("DATE"),
                        )),
                    p().child(env!("UNAME")),
                ))
        ))
        // darkmode toggle
        .class(move || darkmode.is_dark().then_some("dark"))
}
