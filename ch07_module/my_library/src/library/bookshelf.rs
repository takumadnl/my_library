use super::book::Book;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

pub struct Bookshelf {
    books: Vec<Book>,
    matcher: SkimMatcherV2,
}

impl Bookshelf {
    pub fn new() -> Self {
        let matcher = SkimMatcherV2::default();
        Self {
            books: Vec::new(),
            matcher,
        }
    }

    /// 本を追加するメソッド
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    /// 本を本棚から取り出すメソッド
    pub fn remove_book(&mut self, _book: &Book) -> Option<Book> {
        todo!("implement `Bookshelf::remove_book`");
    }

    /// 本棚の本をすべて取り出すメソッド
    pub fn take_all_books(&mut self) -> Vec<Book> {
        todo!("implement `Bookshelf::take_all_books`");
    }

    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| self.matcher.fuzzy_match(&book.title, title_query).is_some())
            .collect()
    }

    pub fn search_books_exact(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.title == title_query)
            .collect()
    }

    pub fn search_books_partial(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.title.contains(title_query))
            .collect()
    }
}

impl Default for Bookshelf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::library::book::Book;

    use super::Bookshelf;

    #[test]
    fn test_bookshelf() {
        let mut shelf = Bookshelf::new();
        let book1 = Book::new("すごいぞChatGPT!AIを使って学ぼうRust!", "山田太郎");
        let book2 = Book::new("Pythonプログラミング入門", "山田花子");

        shelf.add_book(book1);
        shelf.add_book(book2);

        let found_books = shelf.search_books("chatgpt");
        println!("{:?}", found_books);
    }
}
