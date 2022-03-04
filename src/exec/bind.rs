#[cfg(feature = "sqlx")]
use sqlx::database::Database;

#[cfg(feature = "sqlx")]
use crate::value::Value;

/// A trait to unify three types:
///
/// 1. [`Query`](sqlx::query::Query)
/// 2. [`QueryAs`](sqlx::query::QueryAs)
/// 3. [`QueryScalar`](sqlx::query::QueryScalar)
#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
pub trait Query<'q, DB>: Sized
where
    DB: sqlx::Database,
{
    fn bind_fn<T>() -> fn(Self, T) -> Self
    where
        T: 'q + Send + sqlx::Encode<'q, DB> + sqlx::Type<DB>;

    fn bind<T>(self, value: T) -> Self
    where
        T: 'q + Send + sqlx::Encode<'q, DB> + sqlx::Type<DB>,
    {
        Self::bind_fn()(self, value)
    }
}

macro_rules! gen_query_common {
    ($feature:expr, $database:ident) => {
        #[cfg(feature = $feature)]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        impl<'q> $crate::exec::bind::Query<'q, ::sqlx::$database>
            for ::sqlx::query::Query<
                'q,
                ::sqlx::$database,
                <::sqlx::$database as ::sqlx::database::HasArguments<'q>>::Arguments,
            >
        {
            fn bind_fn<T>() -> fn(Self, T) -> Self
            where
                T: 'q
                    + ::std::marker::Send
                    + ::sqlx::Encode<'q, sqlx::$database>
                    + ::sqlx::Type<sqlx::$database>,
            {
                ::sqlx::query::Query::bind
            }
        }

        #[cfg(feature = $feature)]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        impl<'q, O> $crate::exec::bind::Query<'q, ::sqlx::$database>
            for ::sqlx::query::QueryAs<
                'q,
                ::sqlx::$database,
                O,
                <::sqlx::$database as ::sqlx::database::HasArguments<'q>>::Arguments,
            >
        {
            fn bind_fn<T>() -> fn(Self, T) -> Self
            where
                T: 'q
                    + ::std::marker::Send
                    + ::sqlx::Encode<'q, ::sqlx::$database>
                    + ::sqlx::Type<sqlx::$database>,
            {
                ::sqlx::query::QueryAs::bind
            }
        }

        #[cfg(feature = $feature)]
        #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
        impl<'q, O> $crate::exec::bind::Query<'q, ::sqlx::$database>
            for ::sqlx::query::QueryScalar<
                'q,
                ::sqlx::$database,
                O,
                <::sqlx::$database as ::sqlx::database::HasArguments<'q>>::Arguments,
            >
        {
            fn bind_fn<T>() -> fn(Self, T) -> Self
            where
                T: 'q
                    + ::std::marker::Send
                    + ::sqlx::Encode<'q, ::sqlx::$database>
                    + ::sqlx::Type<sqlx::$database>,
            {
                ::sqlx::query::QueryScalar::bind
            }
        }
    };
}

gen_query_common!("postgres", Postgres);
gen_query_common!("mysql", MySql);
gen_query_common!("sqlite", Sqlite);

/// Provide a [`Database`](sqlx::database::Database) a bind method for binding
/// [`Value`](crate::value::Value) into [`Query`](sqlx::query::Query),
/// [`QueryAs`](sqlx::query::QueryAs) or
/// [`QueryScalar`](sqlx::query::QueryScalar).
#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
pub trait Bind: Database {
    fn bind<'q, Q>(query: Q, value: Value<'q>) -> Result<Q, sqlx::Error>
    where
        Q: Query<'q, Self>;
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

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
impl Bind for sqlx::Postgres {
    fn bind<'q, Q>(query: Q, value: Value<'q>) -> Result<Q, sqlx::Error>
    where
        Q: Query<'q, Self>,
    {
        match value {
            Value::Bool(val) => Ok(query.bind(val)),
            Value::TinyInt(val) => Ok(query.bind(val)),
            Value::SmallInt(val) => Ok(query.bind(val)),
            Value::Int(val) => Ok(query.bind(val)),
            Value::BigInt(val) => Ok(query.bind(val)),
            Value::TinyUInt(..) => Err(unsupported::<sqlx::Postgres, u8>()),
            Value::SmallUInt(..) => Err(unsupported::<sqlx::Postgres, u16>()),
            Value::UInt(..) => Err(unsupported::<sqlx::Postgres, u32>()),
            Value::BigUInt(..) => Err(unsupported::<sqlx::Postgres, u64>()),
            Value::Text(val) => Ok(query.bind(val)),
            Value::Bytes(val) => Ok(query.bind(val)),
            Value::Null(crate::value::Null::Bool(..)) => Ok(query.bind(None::<bool>)),
            Value::Null(crate::value::Null::TinyInt(..)) => Ok(query.bind(None::<i8>)),
            Value::Null(crate::value::Null::SmallInt(..)) => Ok(query.bind(None::<i16>)),
            Value::Null(crate::value::Null::Int(..)) => Ok(query.bind(None::<i32>)),
            Value::Null(crate::value::Null::BigInt(..)) => Ok(query.bind(None::<i64>)),
            Value::Null(crate::value::Null::TinyUInt(..)) => {
                Err(unsupported::<sqlx::Postgres, u8>())
            }
            Value::Null(crate::value::Null::SmallUInt(..)) => {
                Err(unsupported::<sqlx::Postgres, u16>())
            }
            Value::Null(crate::value::Null::UInt(..)) => Err(unsupported::<sqlx::Postgres, u32>()),
            Value::Null(crate::value::Null::BigUInt(..)) => {
                Err(unsupported::<sqlx::Postgres, u64>())
            }
            Value::Null(crate::value::Null::Text(..)) => Ok(query.bind(None::<&'q str>)),
            Value::Null(crate::value::Null::Bytes(..)) => Ok(query.bind(None::<&'q str>)),
        }
    }
}

#[cfg(feature = "mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
impl Bind for sqlx::MySql {
    fn bind<'q, Q>(query: Q, value: Value<'q>) -> Result<Q, sqlx::Error>
    where
        Q: Query<'q, Self>,
    {
        match value {
            Value::Bool(val) => Ok(query.bind(val)),
            Value::TinyInt(val) => Ok(query.bind(val)),
            Value::SmallInt(val) => Ok(query.bind(val)),
            Value::Int(val) => Ok(query.bind(val)),
            Value::BigInt(val) => Ok(query.bind(val)),
            Value::TinyUInt(val) => Ok(query.bind(val)),
            Value::SmallUInt(val) => Ok(query.bind(val)),
            Value::UInt(val) => Ok(query.bind(val)),
            Value::BigUInt(val) => Ok(query.bind(val)),
            Value::Text(val) => Ok(query.bind(val)),
            Value::Bytes(val) => Ok(query.bind(val)),
            Value::Null(crate::value::Null::Bool(..)) => Ok(query.bind(None::<bool>)),
            Value::Null(crate::value::Null::TinyInt(..)) => Ok(query.bind(None::<i8>)),
            Value::Null(crate::value::Null::SmallInt(..)) => Ok(query.bind(None::<i16>)),
            Value::Null(crate::value::Null::Int(..)) => Ok(query.bind(None::<i32>)),
            Value::Null(crate::value::Null::BigInt(..)) => Ok(query.bind(None::<i64>)),
            Value::Null(crate::value::Null::TinyUInt(..)) => Ok(query.bind(None::<u8>)),
            Value::Null(crate::value::Null::SmallUInt(..)) => Ok(query.bind(None::<u16>)),
            Value::Null(crate::value::Null::UInt(..)) => Ok(query.bind(None::<u32>)),
            Value::Null(crate::value::Null::BigUInt(..)) => Ok(query.bind(None::<u64>)),
            Value::Null(crate::value::Null::Text(..)) => Ok(query.bind(None::<&'q str>)),
            Value::Null(crate::value::Null::Bytes(..)) => Ok(query.bind(None::<&'q str>)),
        }
    }
}

#[cfg(feature = "sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
impl Bind for sqlx::Sqlite {
    fn bind<'q, Q>(query: Q, value: Value<'q>) -> Result<Q, sqlx::Error>
    where
        Q: Query<'q, Self>,
    {
        match value {
            Value::Bool(val) => Ok(query.bind(val)),
            Value::TinyInt(val) => Ok(query.bind(val)),
            Value::SmallInt(val) => Ok(query.bind(val)),
            Value::Int(val) => Ok(query.bind(val)),
            Value::BigInt(val) => Ok(query.bind(val)),
            Value::TinyUInt(val) => Ok(query.bind(val)),
            Value::SmallUInt(val) => Ok(query.bind(val)),
            Value::UInt(val) => Ok(query.bind(val)),
            Value::BigUInt(..) => Err(unsupported::<sqlx::Sqlite, u64>()),
            Value::Text(val) => Ok(query.bind(val)),
            Value::Bytes(val) => Ok(query.bind(val)),
            Value::Null(crate::value::Null::Bool(..)) => Ok(query.bind(None::<bool>)),
            Value::Null(crate::value::Null::TinyInt(..)) => Ok(query.bind(None::<i8>)),
            Value::Null(crate::value::Null::SmallInt(..)) => Ok(query.bind(None::<i16>)),
            Value::Null(crate::value::Null::Int(..)) => Ok(query.bind(None::<i32>)),
            Value::Null(crate::value::Null::BigInt(..)) => Ok(query.bind(None::<i64>)),
            Value::Null(crate::value::Null::TinyUInt(..)) => Ok(query.bind(None::<u8>)),
            Value::Null(crate::value::Null::SmallUInt(..)) => Ok(query.bind(None::<u16>)),
            Value::Null(crate::value::Null::UInt(..)) => Ok(query.bind(None::<u32>)),
            Value::Null(crate::value::Null::BigUInt(..)) => Err(unsupported::<sqlx::Sqlite, u64>()),
            Value::Null(crate::value::Null::Text(..)) => Ok(query.bind(None::<&'q str>)),
            Value::Null(crate::value::Null::Bytes(..)) => Ok(query.bind(None::<&'q str>)),
        }
    }
}
