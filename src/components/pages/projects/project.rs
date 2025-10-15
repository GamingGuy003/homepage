use leptos::prelude::*;

#[allow(dead_code)]
/// data associated with a project
pub struct Project {
    /// the name of the project
    title: String,
    /// the image displayed on the project card
    image: String,
    /// the body of the project article
    body: AnyView,
}

#[allow(dead_code)]
impl Project {
    /// creates a new project struct from the provided data
    pub fn new(title: &str, image: &str, body: AnyView) -> Self {
        Self {
            title: title.to_owned(),
            image: image.to_owned(),
            body,
        }
    }

    /// fetch the title
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    /// getch the image
    pub fn get_image(&self) -> String {
        self.image.clone()
    }

    /// fetch the body
    pub fn get_body(&self) -> &AnyView {
        self.body.as_borrowed()
    }
}
