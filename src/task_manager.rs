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

    pub fn add(&mut self, title: &str) {
        let new_task = Task::new(self.next_id, title);
        println!("追加しました: {}", new_task.title());
        self.tasks.push(new_task);
        self.next_id += 1;
    }

    pub fn list(&self) {
        if self.tasks.is_empty() {
            println!("タスクは0です");
        } else {
            self.tasks.iter().for_each(|task| task.display());
        }
    }

    pub fn complete(&mut self, id: u32) {
        if self.tasks.is_empty() {
            println!("タスクは0です");
            return;
        }

        if let Some(target) = self.tasks.iter_mut().find(|task| task.id() == id) {
            target.done();
            println!(
                "id: {} {} タスクを完了しました",
                target.id(),
                target.title()
            );
        } else {
            println!("該当タスクはありません");
        }
    }

    pub fn delete(&mut self, id: u32) {
        if self.tasks.is_empty() {
            println!("タスクは0です");
            return;
        }
        if let Some(target) = self.tasks.iter_mut().find(|task| task.id() == id) {
            let target_id = target.id();
            let target_title = target.title().to_string();
            self.tasks.retain(|task| task.id() != id);
            println!("id: {} {} タスクを削除しました", target_id, target_title);
        } else {
            println!("該当タスクはありません");
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let tasks_json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(path, tasks_json)?;
        Ok(())
    }

    pub fn load(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        if !std::path::Path::new(path).exists() {
            return Ok(());
        }
        let json = fs::read_to_string(path)?;
        let tasks: Vec<Task> = serde_json::from_str(&json)?;
        if let Some(last_task) = tasks.last() {
            self.next_id = last_task.id() + 1;
        };
        self.tasks = tasks;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Status;

    #[test]
    fn tasks_added() {
        let mut test_task_manager = TaskManager::new();
        let title = "てすと";
        test_task_manager.add(title);

        let test_task = test_task_manager.tasks.first().unwrap();

        assert_eq!(test_task.id(), 1);
        assert_eq!(test_task.title(), "てすと");
    }

    #[test]
    fn task_complete_done() {
        let mut test_task_manager = TaskManager::new();
        let test_task = Task::new(1, "てすと");
        test_task_manager.tasks.push(test_task);
        test_task_manager.complete(1);

        let result = test_task_manager.tasks.first().unwrap();

        assert_eq!(*result.status(), Status::Done);
    }

    #[test]
    fn task_delete() {
        let mut test_task_manager = TaskManager::new();
        let test_task = Task::new(1, "てすと");
        test_task_manager.tasks.push(test_task);
        test_task_manager.delete(1);

        assert!(test_task_manager.tasks.is_empty());
    }

    #[test]
    fn test_save_load() {
        let mut save_task_manager = TaskManager::new();
        let mut load_task_manager = TaskManager::new();

        let test_task = Task::new(1, "てすと");
        save_task_manager.tasks.push(test_task);

        let path = "./test_save_load.json";

        save_task_manager.save(path).unwrap();
        load_task_manager.load(path).unwrap();

        let result = load_task_manager.tasks.first().unwrap();

        assert_eq!(result, save_task_manager.tasks.first().unwrap());

        std::fs::remove_file(path).unwrap();
    }
}
