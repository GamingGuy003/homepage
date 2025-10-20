#[allow(dead_code)]
#[derive(Clone)]
/// data associated with a project
pub struct Project {
    /// the name of the project
    title: String,
    /// the image displayed on the project card
    image: String,
    /// where to redirect to on click
    uri: String,
}

#[allow(dead_code)]
impl Project {
    /// creates a new project struct from the provided data
    pub fn new(title: &str, image: String, uri: String) -> Self {
        Self {
            title: title.to_owned(),
            image,
            uri,
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
    pub fn get_uri(&self) -> String {
        self.uri.clone()
    }
}
