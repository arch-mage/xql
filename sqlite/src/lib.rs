pub use sqlx::sqlite::SqliteRow as Row;

pub use sqlx::error::Error;

pub use sqlx::sqlite::Sqlite;

pub type Query<'q> = sqlx::query::Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>>;

pub type QueryAs<'q, O> = sqlx::query::QueryAs<'q, Sqlite, O, sqlx::sqlite::SqliteArguments<'q>>;

pub type QueryScalar<'q, O> =
    sqlx::query::QueryScalar<'q, Sqlite, O, sqlx::sqlite::SqliteArguments<'q>>;

pub trait Executor<'c>: sqlx::Executor<'c, Database = Sqlite> {}

impl<'c, E> Executor<'c> for E where E: sqlx::Executor<'c, Database = Sqlite> {}

pub trait FromRow<'r>: sqlx::FromRow<'r, Row> {}

impl<'r, R> FromRow<'r> for R where R: sqlx::FromRow<'r, Row> {}

#[inline]
pub fn query(query: &str) -> Query {
    sqlx::query(query)
}

#[inline]
pub fn query_as<O>(query: &str) -> QueryAs<O>
where
    O: for<'r> FromRow<'r>,
{
    sqlx::query_as(query)
}

#[inline]
pub fn query_scalar<O>(query: &str) -> QueryScalar<O>
where
    (O,): for<'r> FromRow<'r>,
{
    sqlx::query_scalar(query)
}
