use crate::printable::Printable;

#[derive(Debug, PartialEq)]
pub enum Genre {
    Novel,
    Technical(String),
    Comic(u32),
}
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
    pub genre: Genre,
}
impl Book {
    pub fn new(title: &str, author: &str, pages: u32, genre: Genre) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            pages,
            genre,
        }
    }

    pub fn summary(&self) -> String {
        match &self.genre {
            Genre::Novel => format!(
                "[小説] {}/{} ({}ページ)",
                self.title, self.author, self.pages
            ),
            Genre::Technical(tech) => format!(
                "[技術書: {}] {}/{} ({}ページ)",
                tech, self.title, self.author, self.pages
            ),
            Genre::Comic(vol) => format!(
                "[漫画: 全{}巻] {}/{} ({}ページ)",
                vol, self.title, self.author, self.pages
            ),
        }
    }
}

impl Printable for Book {
    fn category(&self) -> String {
        match &self.genre {
            Genre::Novel => "[小説]".to_string(),
            Genre::Technical(tech) => format!("[技術書: {}]", tech),
            Genre::Comic(vol) => format!("[漫画: 全{}巻]", vol),
        }
    }

    fn category_select(&self) -> String {
        match &self.genre {
            Genre::Novel => "[小説]".to_string(),
            Genre::Technical(_) => "[技術書]".to_string(),
            Genre::Comic(_) => "[漫画]".to_string(),
        }
    }

    fn display(&self) -> String {
        format!("{}/{} ({}ページ)", self.title, self.author, self.pages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_new() {
        let test_book = make_test_book(Genre::Technical("機械".to_string()));

        assert_eq!(test_book.title, "Rust入門", "タイトルが違います");
    }

    #[test]
    fn test_book_display() {
        let test_book = make_test_book(Genre::Technical("機械".to_string()));

        let result = test_book.display();

        assert_eq!(result, "Rust入門/著者A (12ページ)");
    }

    #[test]
    fn test_book_category_technical() {
        let test_book = make_test_book(Genre::Technical("機械".to_string()));

        let result = test_book.category();

        assert_eq!(result, "[技術書: 機械]", "ジャンルが違います");
    }
    #[test]
    fn test_book_category_novel() {
        let test_book = make_test_book(Genre::Novel);

        let result = test_book.category();

        assert_eq!(result, "[小説]", "ジャンルが違います");
    }
    #[test]
    fn test_book_category_comic() {
        let test_book = make_test_book(Genre::Comic(25));

        let result = test_book.category();

        assert_eq!(result, "[漫画: 全25巻]", "ジャンルが違います");
    }

    fn make_test_book(genre: Genre) -> Book {
        Book::new("Rust入門", "著者A", 12, genre)
    }
}
