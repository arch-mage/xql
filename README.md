# xql

> An SQL query builder for [sqlx][sqlx]. **Work in progress**

## Table of Contents

1. [Basic Query Building](#basic-query-building)
   1. [Insert statement](#insert-statement)
   2. [Select statement](#select-statement)
   3. [Update statement](#update-statement)
   4. [Delete statement](#delete-statement)
2. [Blanket](#blanket)
   1. [Blanket on Expression](#blanket-on-expression)
   2. [Blanket on Table Expression](#blanket-on-table-expression)
   3. [Blanket on `SELECT` and `VALUES` statement](#blanket-on-select-and-values-statement)
3. [Derive](#derive)
4. [Execution](#execution)
5. [Notes on `str` and `String`](#notes-on-str-and-string)

## Basic Query Building

Suppose you have a table like this:

```sql
CREATE TABLE book(
  id      INTEGER PRIMARY KEY,
  title   TEXT NOT NULL,
  author  TEXT,
  lang    TEXT,
  year    SMALLINT
);
```

The CRUD (or ISUD with sql acronym) will be look like this:

### `INSERT` statement.

```rust
let book1 = "The Fellowship of the Rings".to_string();
let auth1 = "J. R. R. Tolkien".to_string();
let book2 = "Dune".to_string();
let auth2 = "Frank Herbret".to_string();
let english = "English".to_string();

let values = [
    (1_i32, &book1, &auth1, &english, 1954_i16),
    (2_i32, &book2, &auth2, &english, 1965_i16),
];
let insert = xql::insert("book", ["id", "title", "author", "lang", "year"])
    .values(values)
    .returning(["id"]);

assert_eq!(
    insert.to_string(),
    "INSERT INTO book(id, title, author, lang, year) VALUES \
    (1, 'The Fellowship of the Rings', 'J. R. R. Tolkien', 'English', 1954), \
    (2, 'Dune', 'Frank Herbret', 'English', 1965) \
    RETURNING id",
);
```

### `SELECT` statement.

```rust
let select = xql::select(["id", "title"])
    .from("book")
    .filter(xql::or(xql::eq("id", 1), xql::eq("id", 2)))
    .order_by(xql::desc("year"));

assert_eq!(
    select.to_string(),
    "SELECT id, title FROM book WHERE id = 1 OR id = 2 ORDER BY year DESC"
);
```

### `UPDATE` statement.

```rust
let author = &"Frank Herbert".to_string();
let update = xql::update("book")
    .set("author", author)
    .filter(xql::eq("id", 2))
    .returning(["id"]);

assert_eq!(
    update.to_string(),
    "UPDATE book SET author = 'Frank Herbert' WHERE id = 2 RETURNING id",
);
```

### `DELETE` statement.

```rust
let delete = xql::delete("book")
    .filter(xql::eq("id", 1))
    .returning(["id", "title"]);

assert_eq!(
    delete.to_string(),
    "DELETE FROM book WHERE id = 1 RETURNING id, title",
);
```

## Blanket

There are some [blanket implementation][blanket-implementation] for traits that defined
in `xql::blanket` to assist query building.

### Blanket on Expression

Most of expr's function defined in `xql::ops` have method of blanket
implementation of `xql::blanket::ExprExt`.

```rust
use xql::blanket::ExprExt;

let cond = "year".greater_than(1900).and("year".less_equal(2000));
assert_eq!(cond.to_string(), "year > 1900 AND year <= 2000");

let query = xql::select(["id"]).from("book").filter(cond);
assert_eq!(query.to_string(), "SELECT id FROM book WHERE year > 1900 AND year <= 2000");
```

Well, that looks verbose. It can't be helped, because using `gt` or `le` will
clash with `PartialOrd` (which can't be disabled even with
`no_implicit_prelude`). This one below will not compile.

```rust,compile_fail
use xql::blanket::ExprExt;

let cond = "year".gt(1900).and("year".le(2000));
```

A work around is to turn the left hand side into `Expr` first or using a table qualified
column reference.

```rust
use xql::expr::Expr;
use xql::blanket::ExprExt;

let year = Expr::from("year");
let qualified = ("book", "year");

let cond = year.gt(1900).and(qualified.le(2000));
assert_eq!(cond.to_string(), "year > 1900 AND book.year <= 2000");
```

### Blanket on Table Expression

`join` family functions have some blanket implementations.

```rust
use xql::blanket::ExprExt;
use xql::blanket::TableExprExt;

let table = "book".join("category", ("book", "category_id").eq(("category", "id")));
assert_eq!(table.to_string(), "book JOIN category ON book.category_id = category.id");
```

### Blanket on `SELECT` and `VALUES` statement

`SELECT` and `VALUES` are the only statements that can use `UNION` family functions.

```rust
use xql::blanket::ResultExt;

let query = xql::select([1, 2]).union(xql::values([(3, 4)]));

assert_eq!(query.to_string(), "SELECT 1, 2 UNION VALUES (3, 4)");
```

In case you're wondering, `ResultExt`'s name came from
`xql::stmt::result::Result` which is an enum of only `Select` and `Values`. Why
`Result`? Well, because naming is hard and it looks good in `Stmt` enum definition:

```rust
enum Stmt {
    Insert,
    Select,
    Update,
    Delete,
    Values,
    Binary,
    Result, // See!? Exactly 6 characters! Perfectly balanced as all things should be!
}
```

## Derive

You can enable `derive` feature to make query building looks shorter or nicer.

```rust
use xql::Schema;

#[derive(Schema)]
struct Book {
    id: i32,
    title: String,
    author: Option<String>,
    lang: Option<String>,
    year: Option<i32>,
}

let shorter = xql::select(Book::columns()).from(Book::table());
assert_eq!(shorter.to_string(), "SELECT book.id, book.title, book.author, book.lang, book.year FROM book");

let nicer = xql::select([Book::id, Book::title, Book::author, Book::lang, Book::year]).from(Book);
assert_eq!(nicer.to_string(), "SELECT book.id, book.title, book.author, book.lang, book.year FROM book");

assert_eq!(shorter, nicer);
```

The table qualified column will turn to unqualified in `INSERT`'s columns or
`UPDATE`'s `SET`.

```rust
use xql::Schema;
use xql::blanket::ExprExt;

#[derive(Schema)]
struct Book {
    id: i32,
    title: String,
    author: Option<String>,
    lang: Option<String>,
    year: Option<i32>,
}

let values = [(&"Dune".to_string(),)];
let insert = xql::insert(Book, [Book::title]).values(values);
assert_eq!(insert.to_string(), "INSERT INTO book(title) VALUES ('Dune')");

let author = "Frank Herbert".to_string();
let update = xql::update(Book).set(Book::author, &author).filter(Book::id.eq(2));
assert_eq!(update.to_string(), "UPDATE book SET author = 'Frank Herbert' WHERE book.id = 2");
```

## Execution

To execute those queries, enable `sqlx` feature and one of `postgres`, `mysql`
or `sqlite` feature.

```rust
#[derive(sqlx::FromRow)]
struct Output {
    id: i32,
    title: String,
}

#[cfg(feature = "postgres")]
async fn execute(pool: sqlx::Pool::<sqlx::Postgres>) -> Result<(), sqlx::Error> {

    // sqlx::query(..).fetch_all
    let query = xql::select(["id", "title"]).from("book");
    let rows = xql::exec::fetch_all(query, &pool).await?;

    // sqlx::query_as(..).fetch_all
    let query = xql::select(["id", "title"]).from("book");
    let rows: Vec<Output> = xql::exec::fetch_all_as(query, &pool).await?;

    // sqlx::query_scalar(..).fetch_all
    let query = xql::select(["id"]).from("book");
    let rows: Vec<i32> = xql::exec::fetch_all_scalar(query, &pool).await?;

    // or in blanket form
    use xql::blanket::StmtExt;

    let rows = xql::select(["id", "title"])
        .from("book")
        .fetch_all(&pool).await?;

    let rows: Vec<Output> = xql::select(["id", "title"])
        .from("book")
        .fetch_all_as(&pool)
        .await?;


    let rows: Vec<i32> = xql::select(["id"])
        .from("book")
        .fetch_all_scalar(&pool).await?;

    Ok(())
}
```

Available variants are: `fetch_one`, `fetch_all`, `fetch_optional` with `_as`,
`_scalar` or no suffix respectively.

## Notes on `str` and `String`

You may notice serveral use of `&"text".to_string()` in the examples above.
That's because `&str` will turn into an identifier while `&String` will turn
into a literal text.

[sqlx]: https://crates.io/crates/sqlx
[blanket-implementation]: https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
