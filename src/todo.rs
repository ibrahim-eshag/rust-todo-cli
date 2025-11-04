#[derive(Debug, PartialEq)]
pub enum Status {
    Pending,
    Done,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Done => write!(f, "done"),
            Status::Pending => write!(f, "todo"),
        }
    }
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
        let suffix = if self.status == Status::Pending {
            ".todo"
        } else {
            ".done"
        };
        self.title = format!("{}{}", self.title.to_lowercase().replace(" ", "-"), suffix);
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
