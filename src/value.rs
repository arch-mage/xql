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

    TinyUInt(u8),
    SmallUInt(u16),
    UInt(u32),
    BigUInt(u64),

    Text(&'a str),
    Bytes(&'a [u8]),

    #[cfg(feature = "use-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "use-chrono")))]
    DateTime(chrono::DateTime<chrono::Utc>),
}

crate::macros::gen_display!(Value<'_>);

/// 👻
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Null<'a> {
    Bool(PhantomData<bool>),
    TinyInt(PhantomData<i8>),
    SmallInt(PhantomData<i16>),
    Int(PhantomData<i32>),
    BigInt(PhantomData<i64>),
    TinyUInt(PhantomData<u8>),
    SmallUInt(PhantomData<u16>),
    UInt(PhantomData<u32>),
    BigUInt(PhantomData<u64>),
    Text(PhantomData<&'a str>),
    Bytes(PhantomData<&'a [u8]>),

    #[cfg(feature = "use-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "use-chrono")))]
    DateTime(PhantomData<chrono::DateTime<chrono::Utc>>),
}

into_value!(
    bool => Bool,
    i8 => TinyInt,
    i16 => SmallInt,
    i32 => Int,
    i64 => BigInt,
    u8 => TinyUInt,
    u16 => SmallUInt,
    u32 => UInt,
    u64 => BigUInt,
);

#[cfg(feature = "use-chrono")]
into_value!(
    chrono::DateTime<chrono::Utc> => DateTime,
);

into_borrowed_value!(
    String => Text,
    Vec<u8> => Bytes,
);
