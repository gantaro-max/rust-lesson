use std::rc::Rc;

pub fn run() {
    let title = Rc::new("Rustプログラミング完全ガイド".to_string());
    println!("タイトル作成後のカウント: {}", Rc::strong_count(&title));
    let _library = Rc::clone(&title);
    println!("図書館追加後のカウント: {}", Rc::strong_count(&title));
    let _user_a = Rc::clone(&title);
    println!("利用者A追加後のカウント: {}", Rc::strong_count(&title));
    {
        let _user_b = Rc::clone(&title);
        println!("利用者B追加後のカウント: {}", Rc::strong_count(&title));
    }
    println!(
        "利用者Bがスコープを抜けた追加後のカウント: {}",
        Rc::strong_count(&title)
    );
}
