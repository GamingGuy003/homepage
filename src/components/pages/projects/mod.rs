use leptos::{html::div, prelude::*};

use crate::{
    based_url,
    components::pages::projects::{
        project::Project,
        project_card::{ProjectCard, ProjectCardProps},
    },
};

mod project;
mod project_card;
pub mod project_view;

use std::sync::OnceLock;

static PROJECTS: OnceLock<Vec<Project>> = OnceLock::new();
fn get_projects() -> &'static Vec<Project> {
    PROJECTS.get_or_init(|| {
        vec![
            Project::new(
                "Assignment 01 - Webpage creation",
                based_url("./static/images/profile.png"),
                based_url("./projects/assignment01.pdf"),
            ),
            Project::new(
                "Assignment 02 - User Feedback",
                based_url("./static/images/users.png"),
                based_url("./projects/assignment02.pdf"),
            ),
        ]
    })
}

#[component]
pub fn Projects() -> impl IntoView {
    let list = div().class("grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5");
    let projects = get_projects();
    let projects_view = projects
        .iter()
        .map(|project| {
            ProjectCard(
                ProjectCardProps::builder()
                    .project(project)
                    .class(None)
                    .build(),
            )
        })
        .collect_view();
    list.child(projects_view)
}
