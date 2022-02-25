use std::marker::PhantomData;

use crate::build::Build;
use crate::build::Dialect;

#[derive(Debug)]
pub struct Query<'a, D> {
    pub sql: String,
    pub args: Vec<crate::value::Value<'a>>,

    // 👻
    database: PhantomData<D>,
    persistent: bool,
}

impl<'a, D> Query<'a, D> {
    pub fn persistent(mut self, persistent: bool) -> Self {
        self.persistent = persistent;
        self
    }
}

impl<'a> crate::stmt::Stmt<'a> {
    pub fn compile<D: Dialect>(self) -> Query<'a, D> {
        let mut sql = String::with_capacity(256);
        let mut args = Vec::with_capacity(20);
        self.build::<D>(&mut sql, &mut args);
        Query {
            database: PhantomData,
            sql,
            args,
            persistent: false,
        }
    }
}

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
fn quote(s: &str, q: char, buff: &mut String) {
    buff.push(q);
    for ch in s.chars() {
        if ch == q {
            buff.push(q);
        }
        buff.push(ch);
    }
    buff.push(q);
}

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
macro_rules! bind {
    ($query:expr, $param:expr) => {
        match $param {
            $crate::value::Value::Null(val) => match val {
                $crate::value::Null::Bool(..) => $query.bind(None::<bool>),
                $crate::value::Null::TinyInt(..) => $query.bind(None::<i8>),
                $crate::value::Null::SmallInt(..) => $query.bind(None::<i16>),
                $crate::value::Null::Int(..) => $query.bind(None::<i32>),
                $crate::value::Null::BigInt(..) => $query.bind(None::<i64>),
                $crate::value::Null::Text(..) => $query.bind(None::<&str>),
            },
            $crate::value::Value::Bool(val) => $query.bind(val),
            $crate::value::Value::TinyInt(val) => $query.bind(val),
            $crate::value::Value::SmallInt(val) => $query.bind(val),
            $crate::value::Value::Int(val) => $query.bind(val),
            $crate::value::Value::BigInt(val) => $query.bind(val),
            $crate::value::Value::Text(val) => $query.bind(val),
        }
    };
}

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
macro_rules! impl_fetch_one {
    ($feature:ident, $Database:ident) => {
        impl<'a> $crate::backend::Query<'a, $feature::$Database> {
            pub async fn fetch_one<'c, E>(
                self,
                executor: E,
            ) -> Result<$feature::Row, $feature::Error>
            where
                E: $feature::Executor<'c>,
            {
                self.args
                    .into_iter()
                    .fold(
                        $feature::query(self.sql.as_str()).persistent(self.persistent),
                        |query, param| bind!(query, param),
                    )
                    .fetch_one(executor)
                    .await
            }

            pub async fn fetch_one_as<'c, O, E>(self, executor: E) -> Result<O, $feature::Error>
            where
                O: Send + Unpin + for<'r> $feature::FromRow<'r>,
                E: $feature::Executor<'c>,
            {
                self.args
                    .into_iter()
                    .fold(
                        $feature::query_as(self.sql.as_str()).persistent(self.persistent),
                        |query, param| bind!(query, param),
                    )
                    .fetch_one(executor)
                    .await
            }

            pub async fn fetch_one_scalar<'c, O, E>(self, executor: E) -> Result<O, $feature::Error>
            where
                O: Send + Unpin + for<'r> $feature::FromRow<'r>,
                (O,): Send + Unpin + for<'r> $feature::FromRow<'r>,
                E: $feature::Executor<'c>,
            {
                self.args
                    .into_iter()
                    .fold(
                        $feature::query_scalar(self.sql.as_str()).persistent(self.persistent),
                        |query, param| bind!(query, param),
                    )
                    .fetch_one(executor)
                    .await
            }
        }
    };
}

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
macro_rules! impl_fetch_optional {
    ($feature:ident, $Database:ident) => {
        impl<'a> $crate::backend::Query<'a, $feature::$Database> {
            pub async fn fetch_optional<'c, E>(
                self,
                executor: E,
            ) -> Result<Option<$feature::Row>, $feature::Error>
            where
                E: $feature::Executor<'c>,
            {
                self.args
                    .into_iter()
                    .fold(
                        $feature::query(self.sql.as_str()).persistent(self.persistent),
                        |query, param| bind!(query, param),
                    )
                    .fetch_optional(executor)
                    .await
            }

            pub async fn fetch_optional_as<'c, O, E>(
                self,
                executor: E,
            ) -> Result<Option<O>, $feature::Error>
            where
                O: Send + Unpin + for<'r> $feature::FromRow<'r>,
                E: $feature::Executor<'c>,
            {
                self.args
                    .into_iter()
                    .fold(
                        $feature::query_as(self.sql.as_str()).persistent(self.persistent),
                        |query, param| bind!(query, param),
                    )
                    .fetch_optional(executor)
                    .await
            }

            pub async fn fetch_optional_scalar<'c, O, E>(
                self,
                executor: E,
            ) -> Result<Option<O>, $feature::Error>
            where
                O: Send + Unpin + for<'r> $feature::FromRow<'r>,
                (O,): Send + Unpin + for<'r> $feature::FromRow<'r>,
                E: $feature::Executor<'c>,
            {
                self.args
                    .into_iter()
                    .fold(
                        $feature::query_scalar(self.sql.as_str()).persistent(self.persistent),
                        |query, param| bind!(query, param),
                    )
                    .fetch_optional(executor)
                    .await
            }
        }
    };
}

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
macro_rules! impl_fetch_all {
    ($feature:ident, $Database:ident) => {
        impl<'a> $crate::backend::Query<'a, $feature::$Database> {
            pub async fn fetch_all<'c, E>(
                self,
                executor: E,
            ) -> Result<Vec<$feature::Row>, $feature::Error>
            where
                E: $feature::Executor<'c>,
            {
                self.args
                    .into_iter()
                    .fold(
                        $feature::query(self.sql.as_str()).persistent(self.persistent),
                        |query, param| bind!(query, param),
                    )
                    .fetch_all(executor)
                    .await
            }

            pub async fn fetch_all_as<'c, O, E>(
                self,
                executor: E,
            ) -> Result<Vec<O>, $feature::Error>
            where
                O: Send + Unpin + for<'r> $feature::FromRow<'r>,
                E: $feature::Executor<'c>,
            {
                self.args
                    .into_iter()
                    .fold(
                        $feature::query_as(self.sql.as_str()).persistent(self.persistent),
                        |query, param| bind!(query, param),
                    )
                    .fetch_all(executor)
                    .await
            }

            pub async fn fetch_all_scalar<'c, O, E>(
                self,
                executor: E,
            ) -> Result<Vec<O>, $feature::Error>
            where
                O: Send + Unpin + for<'r> $feature::FromRow<'r>,
                (O,): Send + Unpin + for<'r> $feature::FromRow<'r>,
                E: $feature::Executor<'c>,
            {
                self.args
                    .into_iter()
                    .fold(
                        $feature::query_scalar(self.sql.as_str()).persistent(self.persistent),
                        |query, param| bind!(query, param),
                    )
                    .fetch_all(executor)
                    .await
            }
        }
    };
}

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
macro_rules! impl_sqlx {
    ($feature:ident, $Database:ident) => {
        impl_fetch_one!($feature, $Database);
        impl_fetch_optional!($feature, $Database);
        impl_fetch_all!($feature, $Database);
    };
}

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
macro_rules! generate_funcs {
    ($feature:ident, $Database:ident) => {
        pub async fn fetch_one<'s, 'c, S, E>(
            executor: E,
            stmt: S,
        ) -> Result<$feature::Row, $feature::Error>
        where
            S: Into<$crate::stmt::Stmt<'s>>,
            E: $feature::Executor<'c>,
        {
            stmt.into()
                .compile::<$feature::$Database>()
                .fetch_one(executor)
                .await
        }

        pub async fn fetch_one_as<'s, 'c, O, S, E>(
            executor: E,
            stmt: S,
        ) -> Result<O, $feature::Error>
        where
            S: Into<$crate::stmt::Stmt<'s>>,
            O: Send + Unpin + for<'r> $feature::FromRow<'r>,
            E: $feature::Executor<'c>,
        {
            stmt.into()
                .compile::<$feature::$Database>()
                .fetch_one_as(executor)
                .await
        }

        pub async fn fetch_one_scalar<'s, 'c, O, S, E>(
            executor: E,
            stmt: S,
        ) -> Result<O, $feature::Error>
        where
            S: Into<$crate::stmt::Stmt<'s>>,
            O: Send + Unpin + for<'r> $feature::FromRow<'r>,
            (O,): Send + Unpin + for<'r> $feature::FromRow<'r>,
            E: $feature::Executor<'c>,
        {
            stmt.into()
                .compile::<$feature::$Database>()
                .fetch_one_scalar(executor)
                .await
        }

        pub async fn fetch_all<'s, 'c, S, E>(
            executor: E,
            stmt: S,
        ) -> Result<Vec<$feature::Row>, $feature::Error>
        where
            S: Into<$crate::stmt::Stmt<'s>>,
            E: $feature::Executor<'c>,
        {
            stmt.into()
                .compile::<$feature::$Database>()
                .fetch_all(executor)
                .await
        }

        pub async fn fetch_all_as<'s, 'c, O, S, E>(
            executor: E,
            stmt: S,
        ) -> Result<Vec<O>, $feature::Error>
        where
            S: Into<$crate::stmt::Stmt<'s>>,
            O: Send + Unpin + for<'r> $feature::FromRow<'r>,
            E: $feature::Executor<'c>,
        {
            stmt.into()
                .compile::<$feature::$Database>()
                .fetch_all_as(executor)
                .await
        }

        pub async fn fetch_all_scalar<'s, 'c, O, S, E>(
            executor: E,
            stmt: S,
        ) -> Result<Vec<O>, $feature::Error>
        where
            S: Into<$crate::stmt::Stmt<'s>>,
            O: Send + Unpin + for<'r> $feature::FromRow<'r>,
            (O,): Send + Unpin + for<'r> $feature::FromRow<'r>,
            E: $feature::Executor<'c>,
        {
            stmt.into()
                .compile::<$feature::$Database>()
                .fetch_all_scalar(executor)
                .await
        }

        pub async fn fetch_optional<'s, 'c, S, E>(
            executor: E,
            stmt: S,
        ) -> Result<Option<$feature::Row>, $feature::Error>
        where
            S: Into<$crate::stmt::Stmt<'s>>,
            E: $feature::Executor<'c>,
        {
            stmt.into()
                .compile::<$feature::$Database>()
                .fetch_optional(executor)
                .await
        }

        pub async fn fetch_optional_as<'s, 'c, O, S, E>(
            executor: E,
            stmt: S,
        ) -> Result<Option<O>, $feature::Error>
        where
            S: Into<$crate::stmt::Stmt<'s>>,
            O: Send + Unpin + for<'r> $feature::FromRow<'r>,
            E: $feature::Executor<'c>,
        {
            stmt.into()
                .compile::<$feature::$Database>()
                .fetch_optional_as(executor)
                .await
        }

        pub async fn fetch_optional_scalar<'s, 'c, O, S, E>(
            executor: E,
            stmt: S,
        ) -> Result<Option<O>, $feature::Error>
        where
            S: Into<$crate::stmt::Stmt<'s>>,
            O: Send + Unpin + for<'r> $feature::FromRow<'r>,
            (O,): Send + Unpin + for<'r> $feature::FromRow<'r>,
            E: $feature::Executor<'c>,
        {
            stmt.into()
                .compile::<$feature::$Database>()
                .fetch_optional_scalar(executor)
                .await
        }
    };
}

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "mysql")]
pub mod mysql;

#[cfg(feature = "sqlite")]
pub mod sqlite;
