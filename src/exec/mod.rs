#[cfg(feature = "sqlx")]
use std::{future::Future, pin::Pin};

#[cfg(feature = "sqlx")]
use sqlx::{Database, Executor, FromRow};

#[cfg(feature = "mysql")]
use sqlx::MySql;
#[cfg(feature = "postgres")]
use sqlx::Postgres;
#[cfg(feature = "sqlite")]
use sqlx::Sqlite;

#[cfg(feature = "sqlx")]
use crate::{build::Dialect, build::ToSql, exec::bind::Bind, stmt::Stmt, value::Value};

pub mod bind;

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

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
pub trait Backend: Database + Bind {
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

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
impl Backend for Postgres {
    gen_methods!();
}

#[cfg(feature = "mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
impl Backend for MySql {
    gen_methods!();
}

#[cfg(feature = "sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
impl Backend for Sqlite {
    gen_methods!();
}

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
pub async fn fetch_one<'c, 'v, DB, E, S>(stmt: S, executor: E) -> Result<DB::Row, sqlx::Error>
where
    S: Into<Stmt<'v>>,
    DB: Backend + Dialect,
    E: Executor<'c, Database = DB>,
{
    let (sql, args) = stmt.into().to_sql::<E::Database>();
    E::Database::fetch_one(executor, sql, args).await
}

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
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

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
pub async fn fetch_all<'c, 'v, DB, E, S>(stmt: S, executor: E) -> Result<Vec<DB::Row>, sqlx::Error>
where
    S: Into<Stmt<'v>>,
    DB: Backend + Dialect,
    E: Executor<'c, Database = DB>,
{
    let (sql, args) = stmt.into().to_sql::<E::Database>();
    E::Database::fetch_all(executor, sql, args).await
}

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
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

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
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

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
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

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
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

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
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

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
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
