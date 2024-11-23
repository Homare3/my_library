use super::book::Book;
use fuzzy_matcher::{skim::{SkimMatcher, SkimMatcherV2}, FuzzyMatcher};

pub struct Bookshelf {
    books: Vec<Book>,
    matcher: SkimMatcherV2,
}
impl Bookshelf {
    pub fn new() -> Self {
        let matcher = SkimMatcherV2::default(); 
        Self {books: Vec::new(), matcher: matcher }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn search_books(&self, title_puery: &str)-> Vec<&Book> {
        self.books.iter().filter(|book| self.matcher.fuzzy_match(&book.get_title(), title_puery).is_some()).collect()
    }
    // タイトル名完全一致を検索
    pub fn search_books_exact(&self, title_puery: &str) -> Vec<&Book> {
        self.books.iter().filter(|book| book.get_title() == title_puery).collect()
    }

    // タイトル名の部分一致
    pub fn search_books_partial(&self, title_query: &str) -> Vec<&Book> {
        self.books.iter().filter(|book| book.get_title().contains(title_query)).collect()
    }

    pub fn remove_book(&mut self, book: &Book) -> Option<Book> {
        todo!("Implement `Bookshelf::remove_book`");
    }

    pub fn take_all_books(&mut self) -> Vec<Book> {
        todo!("Implement `Bookshelf::take_all_book`")
    }

    
}

#[cfg(test)]
mod tests {
    use super::{Book, Bookshelf};

    #[test]
    fn test_bookshelf() {
        let mut shelf = Bookshelf::new();
        let book1 = Book::new("すごいぞChatGPT!AIを使って学ぼうRust！", "山田太郎");
        let book2 = Book::new("Pythonプログラミング入門", "山田花子");
        shelf.add_book(book1);
        shelf.add_book(book2);

        let found_books = shelf.search_books("chatgpt");
        println!("{:?}", found_books);
    }
}