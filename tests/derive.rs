#![allow(dead_code)]

use xql::Schema;

#[derive(Schema)]
struct Book {
    id: i32,
    title: String,
    author: String,
    year: i16,
}

#[allow(clippy::from_over_into)]
impl<'a> std::convert::Into<xql::item::Row<'a>> for &'a Book {
    fn into(self) -> xql::item::Row<'a> {
        let row: [xql::expr::Expr<'a>; 4] = [
            self.id.into(),
            (&self.title).into(),
            (&self.author).into(),
            self.year.into(),
        ];
        row.into()
    }
}

#[allow(clippy::from_over_into)]
impl<'a> std::convert::Into<xql::clause::Set<'a>> for &'a Book {
    fn into(self) -> xql::clause::Set<'a> {
        let sets: [(xql::item::Ident<'a>, xql::expr::Expr<'a>); 4] = [
            (Book::id.into(), self.id.into()),
            (Book::title.into(), (&self.title).into()),
            (Book::author.into(), (&self.author).into()),
            (Book::year.into(), self.year.into()),
        ];
        sets.into()
    }
}

#[test]
fn select() {
    let expect = "SELECT book.id, book.title, book.author, book.year FROM book";
    let query = xql::select([Book::id, Book::title, Book::author, Book::year]).from(Book);
    assert_eq!(query.to_string(), expect);
    let query = xql::select(Book::columns()).from(Book::table());
    assert_eq!(query.to_string(), expect);
}

#[test]
fn insert() {
    let book = Book {
        id: 1,
        title: "Dune".to_string(),
        author: "Frank Herbert".to_string(),
        year: 1965,
    };

    let expect = "INSERT INTO book(id, title, author, year) VALUES (1, 'Dune', 'Frank Herbert', 1965) RETURNING book.id, book.title, book.author, book.year";
    let query = xql::insert(Book, [Book::id, Book::title, Book::author, Book::year])
        .values([&book])
        .returning([Book::id, Book::title, Book::author, Book::year]);
    assert_eq!(query.to_string(), expect);
    let query = xql::insert(Book::table(), Book::columns())
        .values([&book])
        .returning(Book::columns());
    assert_eq!(query.to_string(), expect);
}

#[test]
fn update() {
    let book = Book {
        id: 1,
        title: "Dune".to_string(),
        author: "Frank Herbert".to_string(),
        year: 1965,
    };

    let expect = "UPDATE book SET id = 1, title = 'Dune', author = 'Frank Herbert', year = 1965 RETURNING book.id, book.title, book.author, book.year";
    let query = xql::update(Book).set_values(&book).returning([
        Book::id,
        Book::title,
        Book::author,
        Book::year,
    ]);
    assert_eq!(query.to_string(), expect);
    let query = xql::update(Book::table())
        .set_values(&book)
        .returning(Book::columns());
    assert_eq!(query.to_string(), expect);
}

#[test]
fn delete() {
    let expect = "DELETE FROM book";
    let query = xql::delete(Book);
    assert_eq!(query.to_string(), expect);
    let query = xql::delete(Book::table());
    assert_eq!(query.to_string(), expect);
}
