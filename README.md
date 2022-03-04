# xql

> An SQL query builder for [sqlx][sqlx]. **Work in progress**

# example usages

Suppose you have a table like this:

```sql
CREATE TABLE book(
  id      INTEGER   NOT NULL PRIMARY KEY,
  title   TEXT      NOT NULL,
  year    SMALLINT  NOT NULL,
  author  TEXT
);
```

The CRUD (or ISUD with sql acronym) will be look like this:

1. `INSERT` statement.

```rust
let book1 = "The Fellowship of the Rings".to_string();
let auth1 = "J. R. R. Tolkien".to_string();
let book2 = "Dune".to_string();
let auth2 = "Frank Herbret".to_string();

let insert = xql::insert("book", ["id", "title", "year", "author"])
    .values([
        (1_i32, &book1, 1954_i16, &auth1),
        (2_i32, &book2, 1965_i16, &auth2),
    ])
    .returning(["id"]);

assert_eq!(
    insert.to_string(),
    "INSERT INTO book(id, title, year, author) VALUES \
    (1, 'The Fellowship of the Rings', 1954, 'J. R. R. Tolkien'), \
    (2, 'Dune', 1965, 'Frank Herbret') \
    RETURNING id",
);
```

2. `SELECT` statement.

```rust
use xql::blanket::ExprExt;
use xql::ops::or;

let select = xql::select(["id", "title"])
    .from("book")
    .filter(or(
        "id".equal(1),
        "id".equal(2),
    ))
    .order_by(xql::desc("year"));

assert_eq!(
    select.to_string(),
    "SELECT id, title \
    FROM book \
    WHERE id = 1 OR id = 2 \
    ORDER BY year DESC"
);
```

3. `UPDATE` statement.

```rust
let author = &"Frank Herbert".to_string();
let update = xql::update("book")
    .set("author", author)
    .filter(xql::eq("id", 2))
    .returning(["id"]);

assert_eq!(
    update.to_string(),
    "UPDATE book \
    SET author = 'Frank Herbert' \
    WHERE id = 2 \
    RETURNING id",
);
```

4. `DELETE` statement.

```rust
let delete = xql::delete("book")
    .filter(xql::eq("id", 1))
    .returning(["id", "title"]);

assert_eq!(
    delete.to_string(),
    "DELETE FROM book \
    WHERE id = 1 \
    RETURNING id, title",
);
```

To execute those queries, add [sqlx][sqlx] to dependencies and enable the backend.

```rust
#[cfg(all(feature = "postgres", not(feature = "mysql"), not(feature = "sqlite")))]
async fn execute<'a>(
    pool: sqlx::Pool<sqlx::Postgres>,
    insert: xql::stmt::insert::Insert<'a>,
    select: xql::stmt::select::Select<'a>,
    update: xql::stmt::update::Update<'a>,
    delete: xql::stmt::delete::Delete<'a>,
) -> Result<(), sqlx::Error> {
    xql::exec::fetch_all(insert, &pool).await?;
    xql::exec::fetch_all(select, &pool).await?;
    xql::exec::fetch_all(update, &pool).await?;
    xql::exec::fetch_all(delete, &pool).await?;
    Ok(())
}

#[cfg(all(not(feature = "postgres"), feature = "mysql", not(feature = "sqlite")))]
async fn execute<'a>(
    pool: sqlx::Pool<sqlx::MySql>,
    insert: xql::stmt::insert::Insert<'a>,
    select: xql::stmt::select::Select<'a>,
    update: xql::stmt::update::Update<'a>,
    delete: xql::stmt::delete::Delete<'a>,
) -> Result<(), sqlx::Error> {
    xql::exec::fetch_all(insert, &pool).await?;
    xql::exec::fetch_all(select, &pool).await?;
    xql::exec::fetch_all(update, &pool).await?;
    xql::exec::fetch_all(delete, &pool).await?;
    Ok(())
}

#[cfg(all(not(feature = "postgres"), not(feature = "mysql"), feature = "sqlite"))]
async fn execute<'a>(
    pool: sqlx::Pool<sqlx::Sqlite>,
    insert: xql::stmt::insert::Insert<'a>,
    select: xql::stmt::select::Select<'a>,
    update: xql::stmt::update::Update<'a>,
    delete: xql::stmt::delete::Delete<'a>,
) -> Result<(), sqlx::Error> {
    xql::exec::fetch_all(insert, &pool).await?;
    xql::exec::fetch_all(select, &pool).await?;
    xql::exec::fetch_all(update, &pool).await?;
    xql::exec::fetch_all(delete, &pool).await?;
    Ok(())
}
```

[sqlx]: https://crates.io/crates/sqlx
