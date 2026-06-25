use crate::task_manager::TaskManager;

mod task;
mod task_manager;


fn main() {
    let args:Vec<String> = std::env::args().collect();
    if args.len()<2{
        println!("入力内容が不正です"); 
        return;       
    }

    let mut task_manager = TaskManager::new();

    match task_manager.load("./data.json"){
        Ok(_)=>println!("読み込み完了!"),
        Err(e)=>print!("{}",e)
    }

    match args[1].trim() {
        "add" => task_manager.add(args[2].trim()),
        "list" => task_manager.list(),
        "done" => {
           match args[2].trim().parse(){
            Ok(id)=> task_manager.complete(id),
            Err(e)=>println!("{}",e)
           }           
        },
        "delete" => {
            match args[2].trim().parse(){
            Ok(id)=> task_manager.delete(id),
            Err(e)=>println!("{}",e)
           }
        },
        _=> println!("無効なコマンドです")        
    }

    match task_manager.save("./data.json"){
        Ok(_)=>println!("書き込み完了!"),
        Err(e)=>print!("{}",e)
    }


   
    
}
