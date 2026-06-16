mod async_task;
mod book;
mod counter;
mod library;
mod list;
mod logger;
mod macros;
mod printable;
mod shared;
mod task;
mod task_manager;

use crate::book::{Book, Genre};
use crate::library::Library;
use crate::list::List;
use crate::printable::Printable;
use crate::printable::{print_item, print_item_other};

#[tokio::main]
async fn main() {
    let book1 = Book::new("まい日記", "山本山", 12, Genre::Novel);
    let book2 = Book::new("ゆあ日記", "川上川", 21, Genre::Comic(12));
    let book3 = Book::new(
        "あわ日記",
        "空青空",
        33,
        Genre::Technical("農業".to_string()),
    );
    let book4 = Book::new("ぜあ日記", "海碧海", 35, Genre::Novel);

    let mut library = Library::new();
    library.add(book1);
    library.add(book2);
    library.add(book3);
    library.add(book4);

    library.list_all();

    let map1 = library.count_by_genre();

    let map2 = library.count_by_genre_iter();

    if let Some(count) = map1.get("小説") {
        println!("小説:{}種類", count);
    }

    if let Some(count) = map2.get("漫画") {
        println!("漫画:{}種類", count);
    }

    if let Some(check) = library.find_by_title("がう") {
        println!("{}", check.summary());
    } else {
        println!("該当ありません");
    }
    if let Some(check) = library.find_by_title("まい日記") {
        println!("{}", check.summary());
    } else {
        println!("該当ありません");
    }

    let book5 = Book::new("ぜあ日記", "海碧海", 35, Genre::Novel);

    match library.add_with_validation(book5) {
        Ok(()) => println!("登録完了"),
        Err(msg) => println!("{}", msg),
    }

    match library.find_by_title_result("あわ") {
        Ok(book) => println!("{}", book.summary()),
        Err(msg) => println!("{}", msg),
    }

    let book6 = Book::new(
        "がい日記",
        "岩中岩",
        10,
        Genre::Technical("機械".to_string()),
    );
    println!("{}", &book6.full_info());
    print_item(&book6);
    print_item_other(&book6);

    let list = List::Cons(
        3,
        Box::new(List::Cons(4, Box::new(List::Cons(5, Box::new(List::Nil))))),
    );
    println!("合計:{}", list.sum());

    shared::run();

    logger::run();

    counter::run();

    async_task::run().await;

    let map = hashmap! {
        "Alice" => 100,
        "Bob"   => 85,
        "Carol" => 92
    };
    println!("{:?}", map);
}
