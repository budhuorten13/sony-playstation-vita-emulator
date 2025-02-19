use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
    title: String,
    file_path: String,
}

impl Game {
    pub fn new(title: String, file_path: String) -> Self {
        Game { title, file_path }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }
}