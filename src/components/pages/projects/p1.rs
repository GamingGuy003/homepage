use leptos::prelude::*;

use crate::components::pages::projects::{
    project::Project,
    project_card::{ProjectCard, ProjectCardProps},
};

#[component]
pub fn Project1() -> impl IntoView {
    let project_1 = Project::new(
        "Testproject",
        "./static/images/profile.png",
        "oje".into_any(),
    );

    ProjectCard(
        ProjectCardProps::builder()
            .project(project_1)
            .class(None)
            .build(),
    )
}
