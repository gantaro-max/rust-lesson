use crate::printable::Printable;
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
    fn display(&self) -> String {
        format!("{}/{} ({}ページ)", self.title, self.author, self.pages)
    }
}
