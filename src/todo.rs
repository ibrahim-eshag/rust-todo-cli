#[derive(Debug, PartialEq)]
pub enum Status {
    Pending,
    Done,
}

#[derive(Debug)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub status: Status,
}

impl Todo {
    // convert title to slug
    pub fn normalize_title(&mut self) {
        self.title = self.title.to_lowercase().replace(" ", "-");
    }

    pub fn denormalize_title(&mut self) {
        self.title = self.title.replace("-", " ");
    }

    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            status: Status::Pending,
        }
    }

    pub fn change_status(&mut self, status: Status) {
        self.status = status;
    }
}
