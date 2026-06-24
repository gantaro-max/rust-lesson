use crate::task::Task;
use std::{error::Error, fs};

pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: vec![],
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: String) {
        let new_task = Task::new(self.next_id, title);
        println!("追加しました: {}", new_task.title());
        self.tasks.push(new_task);
        self.next_id += 1;
    }

    pub fn list(&self) {
        if self.tasks.is_empty() {
            println!("タスクはありません");
        } else {
            self.tasks.iter().for_each(|task| task.display());
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let tasks_json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(path, tasks_json)?;
        Ok(())
    }

    pub fn load(&mut self,path:&str)->Result<(),Box<dyn Error>>{
        let json = fs::read_to_string(path)?;
        let tasks:Vec<Task> = serde_json::from_str(&json)?;
        self.next_id=tasks.len() as u32+1;
        self.tasks=tasks;
        Ok(())
    }
}
