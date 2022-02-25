pub use sqlx::mysql::MySqlRow as Row;

pub use sqlx::error::Error;

pub use sqlx::mysql::MySql;

pub type Query<'q> = sqlx::query::Query<'q, MySql, sqlx::mysql::MySqlArguments>;

pub type QueryAs<'q, O> = sqlx::query::QueryAs<'q, MySql, O, sqlx::mysql::MySqlArguments>;

pub type QueryScalar<'q, O> = sqlx::query::QueryScalar<'q, MySql, O, sqlx::mysql::MySqlArguments>;

pub trait Executor<'c>: sqlx::Executor<'c, Database = MySql> {}

impl<'c, E> Executor<'c> for E where E: sqlx::Executor<'c, Database = MySql> {}

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
