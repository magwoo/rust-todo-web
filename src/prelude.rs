use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    is_done: bool,
}

impl Task {
    pub fn new(title: impl Into<String>) -> Self {
        Task {
            title: title.into(),
            is_done: false,
        }
    }

    pub fn toggle(&mut self) {
        self.is_done = match self.is_done {
            true => false,
            false => true,
        };
    }

    pub fn done(&mut self) {
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }
}
