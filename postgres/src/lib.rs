pub use sqlx::postgres::PgRow as Row;

pub use sqlx::error::Error;

pub use sqlx::postgres::Postgres;

pub type Query<'q> = sqlx::query::Query<'q, Postgres, sqlx::postgres::PgArguments>;

pub type QueryAs<'q, O> = sqlx::query::QueryAs<'q, Postgres, O, sqlx::postgres::PgArguments>;

pub type QueryScalar<'q, O> =
    sqlx::query::QueryScalar<'q, Postgres, O, sqlx::postgres::PgArguments>;

pub trait Executor<'c>: sqlx::Executor<'c, Database = Postgres> {}

impl<'c, E> Executor<'c> for E where E: sqlx::Executor<'c, Database = Postgres> {}

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
