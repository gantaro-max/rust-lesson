use crate::{book::Book, printable::Printable};
use std::collections::HashMap;

pub struct Library<T: Printable> {
    books: Vec<T>,
}
impl<T: Printable> Library<T> {
    pub fn new() -> Self {
        Self { books: vec![] }
    }
    pub fn add(&mut self, item: T) {
        self.books.push(item);
    }
    pub fn list_all(&self) {
        self.books
            .iter()
            .for_each(|item| println!("{}", item.full_info()));
    }
    pub fn count_by_genre(&self) -> HashMap<String, u32> {
        let mut map: HashMap<String, u32> = HashMap::new();
        for item in &self.books {
            let key = item.category_select();
            *map.entry(key).or_insert(0) += 1;
        }
        map
    }
    pub fn count_by_genre_iter(&self) -> HashMap<String, u32> {
        self.books.iter().fold(HashMap::new(), |mut map, item| {
            let key = item.category_select();
            *map.entry(key).or_insert(0) += 1;
            map
        })
    }
}

impl Library<Book> {
    pub fn find_by_title(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|book| book.title == title)
    }

    pub fn find_by_title_result(&self, title: &str) -> Result<&Book, String> {
        self.books
            .iter()
            .find(|book| book.title == title)
            .ok_or(format!("タイトル「{}」は見つかりませんでした", title))
    }

    pub fn add_with_validation(&mut self, book: Book) -> Result<(), String> {
        if self.books.iter().any(|mb| mb.title == book.title) {
            Err(format!("{}:タイトルが重複しています", book.title))
        } else {
            self.add(book);
            Ok(())
        }
    }
}
