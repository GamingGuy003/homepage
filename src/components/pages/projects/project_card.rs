use leptos::prelude::*;

use leptos::html::{a, div, img};

use crate::components::pages::projects::project::Project;

#[component]
pub fn ProjectCard<'a>(project: &'a Project, class: Option<&'a str>) -> impl IntoView {
    a()
        .href(project.get_uri())
        .class(format!(
        "
            rounded-md text-center h-full dark:bg-background-ui-dark
            shadow-xl transition duration-100 ease-in-out hover:scale-102
            hover:bg-gradient-to-br
            from-terminal-yellow-dark dark:from-terminal-purple-dark
            via-terminal-orange dark:via-none
            to-terminal-red-dark dark:to-terminal-azure-dark
            p-0.75 {}",
        class.unwrap_or_default()
    ))
    .child(
        div()
            .class(
                "relative group h-full flex flex-col rounded-sm shadow-md
                bg:bg-background-content dark:bg-background-content-dark overflow-hidden",
            )
            .child((
                img().src(project.get_image()).alt(project.get_title()),
                div()
                    .class(
                        "absolute bottom-0 left-0 w-full transition-all duration-250 ease-in-out
                        group-hover:h-full group-hover:text-xl group-hover:font-bold
                        bg-background-ui/50 dark:bg-background-content-dark/50 backdrop-blur-md
                        py-3 flex items-center justify-center",
                    )
                    .child(project.get_title()),
            )),
    )
}
