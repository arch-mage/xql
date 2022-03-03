use std::future::Future;
use std::pin::Pin;

use sqlx::database::HasArguments;
use sqlx::query::Query;
use sqlx::query::QueryAs;
use sqlx::query::QueryScalar;
use sqlx::Database;
use sqlx::Executor;
use sqlx::FromRow;

#[cfg(feature = "mysql")]
use xql_sqlx_mysql::MySql;
#[cfg(feature = "postgres")]
use xql_sqlx_postgres::Postgres;
#[cfg(feature = "sqlite")]
use xql_sqlx_sqlite::Sqlite;

use crate::build::Dialect;
use crate::build::ToSql;
use crate::stmt::Stmt;
use crate::value::Value;

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
use crate::value::Null;

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
fn quote(buff: &mut String, val: &str, q: char) {
    buff.push(q);
    for ch in val.chars() {
        if ch == q {
            buff.push(q);
        }
        buff.push(ch);
    }
    buff.push(q);
}

#[cfg(feature = "postgres")]
impl Dialect for Postgres {
    fn quote_literal(val: &str, buff: &mut String) {
        quote(buff, val, '\'')
    }

    fn quote_ident(val: &str, buff: &mut String) {
        quote(buff, val, '"')
    }

    fn bind_param<'a>(n: usize, val: Value<'a>, buff: &mut String) -> Value<'a> {
        buff.push('$');
        buff.push_str(n.to_string().as_str());
        val
    }
}

#[cfg(feature = "mysql")]
impl Dialect for MySql {
    fn quote_literal(val: &str, buff: &mut String) {
        quote(buff, val, '\'')
    }

    fn quote_ident(val: &str, buff: &mut String) {
        quote(buff, val, '`')
    }

    fn bind_param<'a>(_: usize, val: Value<'a>, buff: &mut String) -> Value<'a> {
        buff.push('?');
        val
    }
}

#[cfg(feature = "sqlite")]
impl Dialect for Sqlite {
    fn quote_literal(val: &str, buff: &mut String) {
        quote(buff, val, '\'')
    }

    fn quote_ident(val: &str, buff: &mut String) {
        quote(buff, val, '"')
    }

    fn bind_param<'a>(_: usize, val: Value<'a>, buff: &mut String) -> Value<'a> {
        buff.push('?');
        val
    }
}

#[cfg(any(feature = "postgres", feature = "sqlite"))]
fn unsupported<DB, T>() -> sqlx::Error {
    sqlx::Error::Io(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!(
            "sqlx::encode::Encode<'_, {}> is not implemented for {}",
            std::any::type_name::<T>(),
            std::any::type_name::<DB>(),
        ),
    ))
}

pub trait Bind<'q>: Sized {
    fn bind(self, value: Value<'q>) -> Result<Self, sqlx::Error>;
}

pub trait Backend: Database {
    fn bind_query<'q>(
        query: Query<'q, Self, <Self as HasArguments<'q>>::Arguments>,
        value: Value<'q>,
    ) -> Result<Query<'q, Self, <Self as HasArguments<'q>>::Arguments>, sqlx::Error>;

    fn bind_query_as<'q, O>(
        query: QueryAs<'q, Self, O, <Self as HasArguments<'q>>::Arguments>,
        value: Value<'q>,
    ) -> Result<QueryAs<'q, Self, O, <Self as HasArguments<'q>>::Arguments>, sqlx::Error>;

    fn bind_query_scalar<'q, O>(
        query: QueryScalar<'q, Self, O, <Self as HasArguments<'q>>::Arguments>,
        value: Value<'q>,
    ) -> Result<QueryScalar<'q, Self, O, <Self as HasArguments<'q>>::Arguments>, sqlx::Error>;

    fn fetch_one<'a, 'v: 'a, 'c, E>(
        executor: E,
        query: String,
        args: Vec<Value<'v>>,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<Self::Row, sqlx::Error>>>>
    where
        E: 'a + Executor<'c, Database = Self>;

    #[allow(clippy::type_complexity)]
    fn fetch_optional<'a, 'v: 'a, 'c, E>(
        executor: E,
        query: String,
        args: Vec<Value<'v>>,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<Option<Self::Row>, sqlx::Error>>>>
    where
        E: 'a + Executor<'c, Database = Self>;

    #[allow(clippy::type_complexity)]
    fn fetch_all<'a, 'v: 'a, 'c, E>(
        executor: E,
        query: String,
        args: Vec<Value<'v>>,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<Vec<Self::Row>, sqlx::Error>>>>
    where
        E: 'a + Executor<'c, Database = Self>;

    fn fetch_one_as<'a, 'v: 'a, 'c, O, E>(
        executor: E,
        query: String,
        args: Vec<Value<'v>>,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<O, sqlx::Error>>>>
    where
        E: 'a + Executor<'c, Database = Self>,
        O: Send + Unpin + for<'r> FromRow<'r, Self::Row>;

    fn fetch_optional_as<'a, 'v: 'a, 'c, O, E>(
        executor: E,
        query: String,
        args: Vec<Value<'v>>,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<Option<O>, sqlx::Error>>>>
    where
        E: 'a + Executor<'c, Database = Self>,
        O: Send + Unpin + for<'r> FromRow<'r, Self::Row>;

    fn fetch_all_as<'a, 'v: 'a, 'c, O, E>(
        executor: E,
        query: String,
        args: Vec<Value<'v>>,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<Vec<O>, sqlx::Error>>>>
    where
        E: 'a + Executor<'c, Database = Self>,
        O: Send + Unpin + for<'r> FromRow<'r, Self::Row>;

    fn fetch_one_scalar<'a, 'v: 'a, 'c, O, E>(
        executor: E,
        query: String,
        args: Vec<Value<'v>>,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<O, sqlx::Error>>>>
    where
        E: 'a + Executor<'c, Database = Self>,
        O: Send + Unpin,
        (O,): Send + Unpin + for<'r> FromRow<'r, Self::Row>;

    fn fetch_optional_scalar<'a, 'v: 'a, 'c, O, E>(
        executor: E,
        query: String,
        args: Vec<Value<'v>>,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<Option<O>, sqlx::Error>>>>
    where
        E: 'a + Executor<'c, Database = Self>,
        O: Send + Unpin,
        (O,): Send + Unpin + for<'r> FromRow<'r, Self::Row>;

    fn fetch_all_scalar<'a, 'v: 'a, 'c, O, E>(
        executor: E,
        query: String,
        args: Vec<Value<'v>>,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<Vec<O>, sqlx::Error>>>>
    where
        E: 'a + Executor<'c, Database = Self>,
        O: Send + Unpin,
        (O,): Send + Unpin + for<'r> FromRow<'r, Self::Row>;
}

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
macro_rules! gen_methods {
    () => {
        fn fetch_one<'a, 'v: 'a, 'c, E>(
            executor: E,
            query: String,
            args: Vec<Value<'v>>,
        ) -> Pin<Box<dyn 'a + Future<Output = Result<Self::Row, sqlx::Error>>>>
        where
            E: 'a + Executor<'c, Database = Self>,
        {
            Box::pin(async move {
                args.into_iter()
                    .try_fold(sqlx::query(query.as_str()), Self::bind_query)?
                    .fetch_one(executor)
                    .await
            })
        }

        fn fetch_optional<'a, 'v: 'a, 'c, E>(
            executor: E,
            query: String,
            args: Vec<Value<'v>>,
        ) -> Pin<Box<dyn 'a + Future<Output = Result<Option<Self::Row>, sqlx::Error>>>>
        where
            E: 'a + Executor<'c, Database = Self>,
        {
            Box::pin(async move {
                args.into_iter()
                    .try_fold(sqlx::query(query.as_str()), Self::bind_query)?
                    .fetch_optional(executor)
                    .await
            })
        }

        fn fetch_all<'a, 'v: 'a, 'c, E>(
            executor: E,
            query: String,
            args: Vec<Value<'v>>,
        ) -> Pin<Box<dyn 'a + Future<Output = Result<Vec<Self::Row>, sqlx::Error>>>>
        where
            E: 'a + Executor<'c, Database = Self>,
        {
            Box::pin(async move {
                args.into_iter()
                    .try_fold(sqlx::query(query.as_str()), Self::bind_query)?
                    .fetch_all(executor)
                    .await
            })
        }

        fn fetch_one_as<'a, 'v: 'a, 'c, O, E>(
            executor: E,
            query: String,
            args: Vec<Value<'v>>,
        ) -> Pin<Box<dyn 'a + Future<Output = Result<O, sqlx::Error>>>>
        where
            E: 'a + Executor<'c, Database = Self>,
            O: Send + Unpin + for<'r> FromRow<'r, Self::Row>,
        {
            Box::pin(async move {
                args.into_iter()
                    .try_fold(sqlx::query_as(query.as_str()), Self::bind_query_as)?
                    .fetch_one(executor)
                    .await
            })
        }

        fn fetch_optional_as<'a, 'v: 'a, 'c, O, E>(
            executor: E,
            query: String,
            args: Vec<Value<'v>>,
        ) -> Pin<Box<dyn 'a + Future<Output = Result<Option<O>, sqlx::Error>>>>
        where
            E: 'a + Executor<'c, Database = Self>,
            O: Send + Unpin + for<'r> FromRow<'r, Self::Row>,
        {
            Box::pin(async move {
                args.into_iter()
                    .try_fold(sqlx::query_as(query.as_str()), Self::bind_query_as)?
                    .fetch_optional(executor)
                    .await
            })
        }

        fn fetch_all_as<'a, 'v: 'a, 'c, O, E>(
            executor: E,
            query: String,
            args: Vec<Value<'v>>,
        ) -> Pin<Box<dyn 'a + Future<Output = Result<Vec<O>, sqlx::Error>>>>
        where
            E: 'a + Executor<'c, Database = Self>,
            O: Send + Unpin + for<'r> FromRow<'r, Self::Row>,
        {
            Box::pin(async move {
                args.into_iter()
                    .try_fold(sqlx::query_as(query.as_str()), Self::bind_query_as)?
                    .fetch_all(executor)
                    .await
            })
        }

        fn fetch_one_scalar<'a, 'v: 'a, 'c, O, E>(
            executor: E,
            query: String,
            args: Vec<Value<'v>>,
        ) -> Pin<Box<dyn 'a + Future<Output = Result<O, sqlx::Error>>>>
        where
            E: 'a + Executor<'c, Database = Self>,
            O: Send + Unpin,
            (O,): Send + Unpin + for<'r> FromRow<'r, Self::Row>,
        {
            Box::pin(async move {
                args.into_iter()
                    .try_fold(sqlx::query_scalar(query.as_str()), Self::bind_query_scalar)?
                    .fetch_one(executor)
                    .await
            })
        }

        fn fetch_optional_scalar<'a, 'v: 'a, 'c, O, E>(
            executor: E,
            query: String,
            args: Vec<Value<'v>>,
        ) -> Pin<Box<dyn 'a + Future<Output = Result<Option<O>, sqlx::Error>>>>
        where
            E: 'a + Executor<'c, Database = Self>,
            O: Send + Unpin,
            (O,): Send + Unpin + for<'r> FromRow<'r, Self::Row>,
        {
            Box::pin(async move {
                args.into_iter()
                    .try_fold(sqlx::query_scalar(query.as_str()), Self::bind_query_scalar)?
                    .fetch_optional(executor)
                    .await
            })
        }

        fn fetch_all_scalar<'a, 'v: 'a, 'c, O, E>(
            executor: E,
            query: String,
            args: Vec<Value<'v>>,
        ) -> Pin<Box<dyn 'a + Future<Output = Result<Vec<O>, sqlx::Error>>>>
        where
            E: 'a + Executor<'c, Database = Self>,
            O: Send + Unpin,
            (O,): Send + Unpin + for<'r> FromRow<'r, Self::Row>,
        {
            Box::pin(async move {
                args.into_iter()
                    .try_fold(sqlx::query_scalar(query.as_str()), Self::bind_query_scalar)?
                    .fetch_all(executor)
                    .await
            })
        }
    };
}

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
macro_rules! gen_binds {
    ($macro:ident) => {
        fn bind_query<'q>(
            query: Query<'q, Self, <Self as HasArguments<'q>>::Arguments>,
            value: Value<'q>,
        ) -> Result<Query<'q, Self, <Self as HasArguments<'q>>::Arguments>, sqlx::Error> {
            $crate::macros::$macro!(query, value)
        }

        fn bind_query_as<'q, O>(
            query: QueryAs<'q, Self, O, <Self as HasArguments<'q>>::Arguments>,
            value: Value<'q>,
        ) -> Result<QueryAs<'q, Self, O, <Self as HasArguments<'q>>::Arguments>, sqlx::Error> {
            $crate::macros::$macro!(query, value)
        }

        fn bind_query_scalar<'q, O>(
            query: QueryScalar<'q, Self, O, <Self as HasArguments<'q>>::Arguments>,
            value: Value<'q>,
        ) -> Result<QueryScalar<'q, Self, O, <Self as HasArguments<'q>>::Arguments>, sqlx::Error> {
            $crate::macros::$macro!(query, value)
        }
    };
}

#[cfg(feature = "postgres")]
impl Backend for Postgres {
    gen_methods!();
    gen_binds!(binding_postgres);
}

#[cfg(feature = "mysql")]
impl Backend for MySql {
    gen_methods!();
    gen_binds!(binding_mysql);
}

#[cfg(feature = "sqlite")]
impl Backend for Sqlite {
    gen_methods!();
    gen_binds!(binding_sqlite);
}

pub async fn fetch_one<'c, 'v, DB, E, S>(stmt: S, executor: E) -> Result<DB::Row, sqlx::Error>
where
    S: Into<Stmt<'v>>,
    DB: Backend + Dialect,
    E: Executor<'c, Database = DB>,
{
    let (sql, args) = stmt.into().to_sql::<E::Database>();
    E::Database::fetch_one(executor, sql, args).await
}

pub async fn fetch_optional<'c, 'v, DB, E, S>(
    stmt: S,
    executor: E,
) -> Result<Option<DB::Row>, sqlx::Error>
where
    S: Into<Stmt<'v>>,
    DB: Backend + Dialect,
    E: Executor<'c, Database = DB>,
{
    let (sql, args) = stmt.into().to_sql::<E::Database>();
    E::Database::fetch_optional(executor, sql, args).await
}

pub async fn fetch_all<'c, 'v, DB, E, S>(stmt: S, executor: E) -> Result<Vec<DB::Row>, sqlx::Error>
where
    S: Into<Stmt<'v>>,
    DB: Backend + Dialect,
    E: Executor<'c, Database = DB>,
{
    let (sql, args) = stmt.into().to_sql::<E::Database>();
    E::Database::fetch_all(executor, sql, args).await
}

pub async fn fetch_one_as<'c, 'v, O, E, S>(stmt: S, executor: E) -> Result<O, sqlx::Error>
where
    S: Into<Stmt<'v>>,
    E: Executor<'c>,
    E::Database: Backend + Dialect,
    O: Send + Unpin + for<'r> FromRow<'r, <E::Database as Database>::Row>,
{
    let (sql, args) = stmt.into().to_sql::<E::Database>();
    E::Database::fetch_one_as(executor, sql, args).await
}

pub async fn fetch_optional_as<'c, 'v, O, E, S>(
    stmt: S,
    executor: E,
) -> Result<Option<O>, sqlx::Error>
where
    S: Into<Stmt<'v>>,
    E: Executor<'c>,
    E::Database: Backend + Dialect,
    O: Send + Unpin + for<'r> FromRow<'r, <E::Database as Database>::Row>,
{
    let (sql, args) = stmt.into().to_sql::<E::Database>();
    E::Database::fetch_optional_as(executor, sql, args).await
}

pub async fn fetch_all_as<'c, 'v, O, E, S>(stmt: S, executor: E) -> Result<Vec<O>, sqlx::Error>
where
    S: Into<Stmt<'v>>,
    E: Executor<'c>,
    E::Database: Backend + Dialect,
    O: Send + Unpin + for<'r> FromRow<'r, <E::Database as Database>::Row>,
{
    let (sql, args) = stmt.into().to_sql::<E::Database>();
    E::Database::fetch_all_as(executor, sql, args).await
}

pub async fn fetch_one_scalar<'c, 'v, O, E, S>(stmt: S, executor: E) -> Result<O, sqlx::Error>
where
    S: Into<Stmt<'v>>,
    E: Executor<'c>,
    E::Database: Backend + Dialect,
    O: Send + Unpin,
    (O,): for<'r> FromRow<'r, <E::Database as Database>::Row>,
{
    let (sql, args) = stmt.into().to_sql::<E::Database>();
    E::Database::fetch_one_scalar(executor, sql, args).await
}

pub async fn fetch_optional_scalar<'c, 'v, O, E, S>(
    stmt: S,
    executor: E,
) -> Result<Option<O>, sqlx::Error>
where
    S: Into<Stmt<'v>>,
    E: Executor<'c>,
    E::Database: Backend + Dialect,
    O: Send + Unpin,
    (O,): for<'r> FromRow<'r, <E::Database as Database>::Row>,
{
    let (sql, args) = stmt.into().to_sql::<E::Database>();
    E::Database::fetch_optional_scalar(executor, sql, args).await
}

pub async fn fetch_all_scalar<'c, 'v, O, E, S>(stmt: S, executor: E) -> Result<Vec<O>, sqlx::Error>
where
    S: Into<Stmt<'v>>,
    E: Executor<'c>,
    E::Database: Backend + Dialect,
    O: Send + Unpin,
    (O,): for<'r> FromRow<'r, <E::Database as Database>::Row>,
{
    let (sql, args) = stmt.into().to_sql::<E::Database>();
    E::Database::fetch_all_scalar(executor, sql, args).await
}
