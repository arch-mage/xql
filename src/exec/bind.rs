#[cfg(feature = "sqlx")]
use sqlx::{
    database::Database, database::HasArguments, query::Query, query::QueryAs, query::QueryScalar,
};

#[cfg(feature = "sqlx")]
use crate::value::Value;

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
pub trait Bind: Database {
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
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
impl Bind for sqlx::Postgres {
    gen_binds!(binding_postgres);
}

#[cfg(feature = "mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
impl Bind for sqlx::MySql {
    gen_binds!(binding_mysql);
}

#[cfg(feature = "sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
impl Bind for sqlx::Sqlite {
    gen_binds!(binding_sqlite);
}
