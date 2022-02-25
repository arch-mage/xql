use std::marker::PhantomData;

macro_rules! into_value {
    ($($from:ty => $into:ident,)+) => {
        $(
            impl std::convert::From<$from> for Value<'_> {
                #[inline]
                fn from(val: $from) -> Self {
                    Value::$into(val)
                }
            }

            impl std::convert::From<Option<$from>> for Value<'_>
            {
                #[inline]
                fn from(val: Option<$from>) -> Self {
                    match val {
                        None => Value::Null(Null::$into(std::marker::PhantomData)),
                        Some(val) => val.into(),
                    }
                }
            }
        )+
    };
}

macro_rules! into_borrowed_value {
    ($($from:ty => $into:ident,)+) => {
        $(
            impl<'a> std::convert::From<&'a $from> for Value<'a> {
                #[inline]
                fn from(val: &'a $from) -> Self {
                    Value::$into(val)
                }
            }

            impl<'a> std::convert::From<Option<&'a $from>> for Value<'a>
            {
                #[inline]
                fn from(val: Option<&'a $from>) -> Self {
                    match val {
                        None => Value::Null(Null::$into(std::marker::PhantomData)),
                        Some(val) => val.into(),
                    }
                }
            }
        )+
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value<'a> {
    Null(Null<'a>),
    Bool(bool),
    TinyInt(i8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    Text(&'a str),
}

impl std::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Null(..) => write!(f, "null"),
            Value::Bool(val) => write!(f, "{val}"),
            Value::TinyInt(val) => write!(f, "{val}"),
            Value::SmallInt(val) => write!(f, "{val}"),
            Value::Int(val) => write!(f, "{val}"),
            Value::BigInt(val) => write!(f, "{val}"),
            Value::Text(val) => write!(f, "{}", crate::utils::quote(val, '\'')),
        }
    }
}

/// 👻
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Null<'a> {
    Bool(PhantomData<bool>),
    TinyInt(PhantomData<i8>),
    SmallInt(PhantomData<i16>),
    Int(PhantomData<i32>),
    BigInt(PhantomData<i64>),
    Text(PhantomData<&'a str>),
}

into_value!(
    bool => Bool,
    i8 => TinyInt,
    i16 => SmallInt,
    i32 => Int,
    i64 => BigInt,
);

into_borrowed_value!(
    String => Text,
);
