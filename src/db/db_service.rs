use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

use crate::model::Task;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Db {
    tasks: HashMap<u64, Task>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    pub fn insert_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    pub fn get_task(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    pub fn get_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    pub fn delete_task(&mut self, id: u64) {
        self.tasks.remove(&id);
    }

    pub fn update_task(&mut self, id: u64, task: Task) {
        self.tasks.insert(id, task);
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self.tasks)?;
        writer.flush()?;
        Ok(())
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        if file.metadata()?.len() == 0 {
            return Ok(Db::new());
        }

        let db = serde_json::from_reader(file)?;
        Ok(db)
    }
}
