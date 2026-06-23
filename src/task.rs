use serde::{self, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "todo")]
    ToDo,
    #[serde(rename = "done")]
    Done,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: u32,
    title: String,
    status: Status,
}

impl Task {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            status: Status::ToDo,
        }
    }

    pub fn display(&self) {
        match self.status {
            Status::ToDo => println!("[{}] [ ] {}", &self.id, &self.title),
            Status::Done => println!("[{}] [X] {}", &self.id, &self.title),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}
