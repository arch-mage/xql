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
let values = [
    (
        1_i32,
        &"The Fellowship of the Rings".to_string(),
        1954_i16,
        &"J. R. R. Tolkien".to_string(),
    ),
    (
        2_i32,
        &"Dune".to_string(),
        1965_i16,
        &"Frank Herbret".to_string()
    ),
];
let insert = xql::insert("book", ["id", "title", "year", "author"])
    .values(values)
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
let select = xql::select(["id", "title"])
    .from("book")
    .filter(xql::or(
        xql::eq("id", 1),
        xql::eq("id", 2),
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

```rust no_run
#[cfg(all(feature = "postgres", not(feature = "mysql"), not(feature = "sqlite")))]
async fn execute(pool: sqlx::Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {
    xql::postgres::fetch_all(pool, insert).await?;
    xql::postgres::fetch_all(pool, select).await?;
    xql::postgres::fetch_all(pool, update).await?;
    xql::postgres::fetch_all(pool, delete).await?;
    Ok(());
}

#[cfg(all(not(feature = "postgres"), feature = "mysql", not(feature = "sqlite")))]
async fn execute(pool: sqlx::Pool<sqlx::Mysql>) -> Result<(), sqlx::Error> {
    xql::postgres::fetch_all(pool, insert).await?;
    xql::postgres::fetch_all(pool, select).await?;
    xql::postgres::fetch_all(pool, update).await?;
    xql::postgres::fetch_all(pool, delete).await?;
    Ok(());
}

#[cfg(all(not(feature = "postgres"), not(feature = "mysql"), feature = "sqlite"))]
async fn execute(pool: sqlx::Pool<sqlx::Sqlite>) -> Result<(), sqlx::Error> {
    xql::postgres::fetch_all(pool, insert).await?;
    xql::postgres::fetch_all(pool, select).await?;
    xql::postgres::fetch_all(pool, update).await?;
    xql::postgres::fetch_all(pool, delete).await?;
    Ok(());
}
```

[sqlx]: https://crates.io/crates/sqlx
