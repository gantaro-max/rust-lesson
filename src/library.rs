use crate::book::{Book, Genre};
use std::collections::HashMap;

pub struct Library {
    books: Vec<Book>,
}
impl Library {
    pub fn new() -> Self {
        Self { books: vec![] }
    }
    pub fn add(&mut self, book: Book) {
        self.books.push(book);
    }
    pub fn list_all(&self) {
        self.books
            .iter()
            .for_each(|book| println!("{}", book.summary()));
    }
    pub fn count_by_genre(&self) -> HashMap<String, u32> {
        let mut map: HashMap<String, u32> = HashMap::new();
        for book in &self.books {
            let key = match book.genre {
                Genre::Novel => "小説".to_string(),
                Genre::Technical(_) => "技術書".to_string(),
                Genre::Comic(_) => "漫画".to_string(),
            };
            *map.entry(key).or_insert(0) += 1;
        }
        map
    }
    pub fn count_by_genre_iter(&self) -> HashMap<String, u32> {
        self.books.iter().fold(HashMap::new(), |mut map, book| {
            let key = match book.genre {
                Genre::Novel => "小説".to_string(),
                Genre::Technical(_) => "技術書".to_string(),
                Genre::Comic(_) => "漫画".to_string(),
            };
            *map.entry(key).or_insert(0) += 1;
            map
        })
    }
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
