macro_rules! gen_impl_from_arr {
    ($type:ident[$elem:ident]<$lf:lifetime>) => {
        impl<$lf, T, const N: usize> std::convert::From<[T; N]> for $type<$lf>
        where
            T: Into<$elem<$lf>>,
        {
            #[inline]
            fn from(val: [T; N]) -> Self {
                $type(val.into_iter().map(Into::into).collect())
            }
        }
    };

    ($type:ident[$elem:ty]) => {
        impl<T, const N: usize> std::convert::From<[T; N]> for $type
        where
            T: Into<$elem>,
        {
            #[inline]
            fn from(val: [T; N]) -> Self {
                $type(val.into_iter().map(Into::into).collect())
            }
        }
    };
}

macro_rules! gen_impl_from_vec {
    ($type:ident[$elem:ident]<$lf:lifetime>) => {
        impl<$lf, T> std::convert::From<Vec<T>> for $type<$lf>
        where
            T: Into<$elem<$lf>>,
        {
            #[inline]
            fn from(val: Vec<T>) -> Self {
                $type(val.into_iter().map(Into::into).collect())
            }
        }
    };
    ($type:ident[$elem:ty]) => {
        impl<T> std::convert::From<Vec<T>> for $type
        where
            T: Into<$elem>,
        {
            #[inline]
            fn from(val: Vec<T>) -> Self {
                $type(val.into_iter().map(Into::into).collect())
            }
        }
    };
}

macro_rules! gen_impl_from_tup {
    ($type:ident[$elem:ty]) => {
        crate::macros::gen_impl_from_tup!($type[$elem] {
            Tuple1 {
                (0) -> A
            }
            Tuple2 {
                (0) -> A
                (1) -> B
            }
            Tuple3 {
                (0) -> A
                (1) -> B
                (2) -> C
            }
            Tuple4 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
            }
            Tuple5 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
                (4) -> E
            }
            Tuple6 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
                (4) -> E
                (5) -> F
            }
            Tuple7 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
                (4) -> E
                (5) -> F
                (6) -> G
            }
            Tuple8 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
                (4) -> E
                (5) -> F
                (6) -> G
                (7) -> H
            }
            Tuple9 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
                (4) -> E
                (5) -> F
                (6) -> G
                (7) -> H
                (8) -> I
            }
            Tuple10 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
                (4) -> E
                (5) -> F
                (6) -> G
                (7) -> H
                (8) -> I
                (9) -> J
            }
        });
    };
    ($type:ident[$elem:ty] {$(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+}) => {
        $(
            impl<$($T),+> std::convert::From<($($T,)+)> for $type
            where $($T: Into<$elem>,)+
            {
                #[inline]
                fn from(val: ($($T,)+)) -> Self {
                    $type(vec![$(val.$idx.into()),+])
                }
            }
        )+
    };

    ($type:ident[$elem:ident]<$lf:lifetime>) => {
        crate::macros::gen_impl_from_tup!($type[$elem]<$lf> {
            Tuple1 {
                (0) -> A
            }
            Tuple2 {
                (0) -> A
                (1) -> B
            }
            Tuple3 {
                (0) -> A
                (1) -> B
                (2) -> C
            }
            Tuple4 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
            }
            Tuple5 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
                (4) -> E
            }
            Tuple6 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
                (4) -> E
                (5) -> F
            }
            Tuple7 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
                (4) -> E
                (5) -> F
                (6) -> G
            }
            Tuple8 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
                (4) -> E
                (5) -> F
                (6) -> G
                (7) -> H
            }
            Tuple9 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
                (4) -> E
                (5) -> F
                (6) -> G
                (7) -> H
                (8) -> I
            }
            Tuple10 {
                (0) -> A
                (1) -> B
                (2) -> C
                (3) -> D
                (4) -> E
                (5) -> F
                (6) -> G
                (7) -> H
                (8) -> I
                (9) -> J
            }
        });
    };
    ($type:ident[$elem:ident]<$lf:lifetime> {$(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+}) => {
        $(
            impl<$lf, $($T),+> std::convert::From<($($T,)+)> for $type<$lf>
            where $($T: Into<$elem<$lf>>,)+
            {
                #[inline]
                fn from(val: ($($T,)+)) -> Self {
                    $type(vec![$(val.$idx.into()),+])
                }
            }
        )+
    };

}

macro_rules! gen_display {
    ($type:ty) => {
        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let (sql, ..) =
                    $crate::build::ToSql::to_sql::<$crate::build::Display>(self.clone());
                write!(f, "{sql}")
            }
        }
    };
}

pub(crate) use gen_display;
pub(crate) use gen_impl_from_arr;
pub(crate) use gen_impl_from_tup;
pub(crate) use gen_impl_from_vec;

#[cfg(feature = "postgres")]
macro_rules! binding_postgres {
    ($self:expr, $value: expr) => {
        match $value {
            Value::Bool(val) => Ok($self.bind(val)),
            Value::TinyInt(val) => Ok($self.bind(val)),
            Value::SmallInt(val) => Ok($self.bind(val)),
            Value::Int(val) => Ok($self.bind(val)),
            Value::BigInt(val) => Ok($self.bind(val)),
            Value::TinyUInt(..) => Err(unsupported::<Postgres, u8>()),
            Value::SmallUInt(..) => Err(unsupported::<Postgres, u16>()),
            Value::UInt(..) => Err(unsupported::<Postgres, u32>()),
            Value::BigUInt(..) => Err(unsupported::<Postgres, u64>()),
            Value::Text(val) => Ok($self.bind(val)),
            Value::Bytes(val) => Ok($self.bind(val)),
            Value::Null(Null::Bool(..)) => Ok($self.bind(None::<bool>)),
            Value::Null(Null::TinyInt(..)) => Ok($self.bind(None::<i8>)),
            Value::Null(Null::SmallInt(..)) => Ok($self.bind(None::<i16>)),
            Value::Null(Null::Int(..)) => Ok($self.bind(None::<i32>)),
            Value::Null(Null::BigInt(..)) => Ok($self.bind(None::<i64>)),
            Value::Null(Null::TinyUInt(..)) => Err(unsupported::<Postgres, u8>()),
            Value::Null(Null::SmallUInt(..)) => Err(unsupported::<Postgres, u16>()),
            Value::Null(Null::UInt(..)) => Err(unsupported::<Postgres, u32>()),
            Value::Null(Null::BigUInt(..)) => Err(unsupported::<Postgres, u64>()),
            Value::Null(Null::Text(..)) => Ok($self.bind(None::<&'q str>)),
            Value::Null(Null::Bytes(..)) => Ok($self.bind(None::<&'q str>)),
        }
    };
}
#[cfg(feature = "mysql")]
macro_rules! binding_mysql {
    ($self:expr, $value: expr) => {
        match $value {
            Value::Bool(val) => Ok($self.bind(val)),
            Value::TinyInt(val) => Ok($self.bind(val)),
            Value::SmallInt(val) => Ok($self.bind(val)),
            Value::Int(val) => Ok($self.bind(val)),
            Value::BigInt(val) => Ok($self.bind(val)),
            Value::TinyUInt(val) => Ok($self.bind(val)),
            Value::SmallUInt(val) => Ok($self.bind(val)),
            Value::UInt(val) => Ok($self.bind(val)),
            Value::BigUInt(val) => Ok($self.bind(val)),
            Value::Text(val) => Ok($self.bind(val)),
            Value::Bytes(val) => Ok($self.bind(val)),
            Value::Null(Null::Bool(..)) => Ok($self.bind(None::<bool>)),
            Value::Null(Null::TinyInt(..)) => Ok($self.bind(None::<i8>)),
            Value::Null(Null::SmallInt(..)) => Ok($self.bind(None::<i16>)),
            Value::Null(Null::Int(..)) => Ok($self.bind(None::<i32>)),
            Value::Null(Null::BigInt(..)) => Ok($self.bind(None::<i64>)),
            Value::Null(Null::TinyUInt(..)) => Ok($self.bind(None::<u8>)),
            Value::Null(Null::SmallUInt(..)) => Ok($self.bind(None::<u16>)),
            Value::Null(Null::UInt(..)) => Ok($self.bind(None::<u32>)),
            Value::Null(Null::BigUInt(..)) => Ok($self.bind(None::<u64>)),
            Value::Null(Null::Text(..)) => Ok($self.bind(None::<&'q str>)),
            Value::Null(Null::Bytes(..)) => Ok($self.bind(None::<&'q str>)),
        }
    };
}

#[cfg(feature = "sqlite")]
macro_rules! binding_sqlite {
    ($self:expr, $value: expr) => {
        match $value {
            Value::Bool(val) => Ok($self.bind(val)),
            Value::TinyInt(val) => Ok($self.bind(val)),
            Value::SmallInt(val) => Ok($self.bind(val)),
            Value::Int(val) => Ok($self.bind(val)),
            Value::BigInt(val) => Ok($self.bind(val)),
            Value::TinyUInt(val) => Ok($self.bind(val)),
            Value::SmallUInt(val) => Ok($self.bind(val)),
            Value::UInt(val) => Ok($self.bind(val)),
            Value::BigUInt(..) => Err(unsupported::<Sqlite, u64>()),
            Value::Text(val) => Ok($self.bind(val)),
            Value::Bytes(val) => Ok($self.bind(val)),
            Value::Null(Null::Bool(..)) => Ok($self.bind(None::<bool>)),
            Value::Null(Null::TinyInt(..)) => Ok($self.bind(None::<i8>)),
            Value::Null(Null::SmallInt(..)) => Ok($self.bind(None::<i16>)),
            Value::Null(Null::Int(..)) => Ok($self.bind(None::<i32>)),
            Value::Null(Null::BigInt(..)) => Ok($self.bind(None::<i64>)),
            Value::Null(Null::TinyUInt(..)) => Ok($self.bind(None::<u8>)),
            Value::Null(Null::SmallUInt(..)) => Ok($self.bind(None::<u16>)),
            Value::Null(Null::UInt(..)) => Ok($self.bind(None::<u32>)),
            Value::Null(Null::BigUInt(..)) => Err(unsupported::<Sqlite, u64>()),
            Value::Null(Null::Text(..)) => Ok($self.bind(None::<&'q str>)),
            Value::Null(Null::Bytes(..)) => Ok($self.bind(None::<&'q str>)),
        }
    };
}

#[cfg(feature = "postgres")]
pub(crate) use binding_postgres;

#[cfg(feature = "mysql")]
pub(crate) use binding_mysql;

#[cfg(feature = "sqlite")]
pub(crate) use binding_sqlite;
