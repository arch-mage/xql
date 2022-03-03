use crate::clause;
use crate::item::Row;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Data<'a> {
    Select(Box<crate::stmt::select::Select<'a>>),
    Values(crate::stmt::values::Values<'a>),
    Binary(crate::stmt::binary::Binary<'a>),
}

crate::macros::gen_display!(Data<'_>);

impl std::default::Default for Data<'_> {
    #[inline]
    fn default() -> Self {
        Data::Values(crate::stmt::values::Values::default())
    }
}

impl<'a> std::convert::From<clause::Values<'a>> for Data<'a> {
    #[inline]
    fn from(val: clause::Values<'a>) -> Self {
        Data::Values(crate::stmt::values::Values {
            rows: val,
            ..Default::default()
        })
    }
}

impl<'a> std::convert::From<clause::Select<'a>> for Data<'a> {
    #[inline]
    fn from(val: clause::Select<'a>) -> Self {
        Data::Select(Box::new(crate::stmt::select::Select {
            fields: val,
            ..Default::default()
        }))
    }
}

impl<'a> std::convert::From<crate::stmt::values::Values<'a>> for Data<'a> {
    #[inline]
    fn from(val: crate::stmt::values::Values<'a>) -> Self {
        Data::Values(val)
    }
}

impl<'a> std::convert::From<crate::stmt::select::Select<'a>> for Data<'a> {
    #[inline]
    fn from(val: crate::stmt::select::Select<'a>) -> Self {
        Data::Select(Box::new(val))
    }
}

impl<'a> std::convert::From<Box<crate::stmt::select::Select<'a>>> for Data<'a> {
    #[inline]
    fn from(val: Box<crate::stmt::select::Select<'a>>) -> Self {
        Data::Select(val)
    }
}

impl<'a, R> std::convert::From<Vec<R>> for Data<'a>
where
    R: Into<Row<'a>>,
{
    #[inline]
    fn from(val: Vec<R>) -> Self {
        Data::Values(crate::stmt::values::Values {
            rows: clause::Values(val.into_iter().map(Into::into).collect()),
            ..Default::default()
        })
    }
}

impl<'a, R, const N: usize> std::convert::From<[R; N]> for Data<'a>
where
    R: Into<Row<'a>>,
{
    #[inline]
    fn from(val: [R; N]) -> Self {
        Data::Values(crate::stmt::values::Values {
            rows: clause::Values(val.into_iter().map(Into::into).collect()),
            ..Default::default()
        })
    }
}
