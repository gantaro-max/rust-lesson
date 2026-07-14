use crate::task_manager::TaskManager;

mod task;
mod task_manager;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("入力内容が不正です");
        return;
    }

    let mut task_manager = TaskManager::new();

    match task_manager.load("./data.json") {
        Ok(_) => println!("読み込み完了!"),
        Err(e) => print!("{}", e),
    }

    match args[1].trim() {
        "add" => match args.get(2) {
            Some(task) => task_manager.add(task.trim()),
            None => println!("タスクが入力されていません"),
        },
        "list" => task_manager.list(),
        "done" => match parse_task_id(&args) {
            Ok(id) => task_manager.complete(id),
            Err(e) => println!("{}", e),
        },
        "delete" => match parse_task_id(&args) {
            Ok(id) => task_manager.delete(id),
            Err(e) => println!("{}", e),
        },
        _ => println!("無効なコマンドです"),
    }

    match task_manager.save("./data.json") {
        Ok(_) => println!("書き込み完了!"),
        Err(e) => print!("{}", e),
    }
}

fn parse_task_id(args: &[String]) -> Result<u32, String> {
    args.get(2)
        .ok_or_else(|| "値がありません".to_string())
        .and_then(|s| s.parse::<u32>().map_err(|e| e.to_string()))
}
