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

    pub fn complete(&mut self,id:u32){
        if self.tasks.is_empty() {
            println!("タスクは0です");
            return;
        }
        
        if let Some(target) = self.tasks.iter_mut().find(|task|task.id()==id){
            target.done();
            println!("id: {} {} タスクを完了しました",target.id(),target.title());
        }
        else{
            println!("該当タスクはありません");
        }
    }

    pub fn delete(&mut self,id:u32){
        if self.tasks.is_empty() {
            println!("タスクは0です");
            return;
        }
        if let Some(target) = self.tasks.iter_mut().find(|task|task.id()==id){
            let target_id= target.id();
            let target_title= target.title().to_string();
            let new_tasks = self.tasks.iter().filter(|task| task.id()!=target_id).cloned().collect();
            self.tasks=new_tasks;
            println!("id: {} {} タスクを削除しました",target_id,target_title);
            
        }
        else{
            println!("該当タスクはありません");
        }

    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let tasks_json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(path, tasks_json)?;
        Ok(())
    }

    pub fn load(&mut self,path:&str)->Result<(),Box<dyn Error>>{
        if !std::path::Path::new(path).exists(){            
            return Ok(());
        }       
        let json = fs::read_to_string(path)?;
        let tasks:Vec<Task> = serde_json::from_str(&json)?;
        if let Some(last_task) = tasks.last(){
            self.next_id=last_task.id()+1;
        };
        self.tasks = tasks;
        Ok(())
    }
}
