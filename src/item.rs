use crate::expr::Expr;
use crate::table_expr::TableExpr;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ident<'a>(pub &'a str);

crate::macros::gen_display!(Ident<'_>);

impl<'a> std::convert::From<&'a str> for Ident<'a> {
    #[inline]
    fn from(val: &'a str) -> Self {
        Ident(val)
    }
}

impl<'a> std::convert::From<ColumnRef<'a>> for Ident<'a> {
    #[inline]
    fn from(val: ColumnRef<'a>) -> Self {
        match val {
            ColumnRef::Column(col) => col,
            ColumnRef::TableColumn(.., col) => col,
            ColumnRef::SchemaTableColumn(.., col) => col,
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnRef<'a> {
    Column(Ident<'a>),
    TableColumn(Ident<'a>, Ident<'a>),
    SchemaTableColumn(Ident<'a>, Ident<'a>, Ident<'a>),
}

crate::macros::gen_display!(ColumnRef<'_>);

impl std::default::Default for ColumnRef<'_> {
    #[inline]
    fn default() -> Self {
        ColumnRef::Column(Default::default())
    }
}

impl<'a> std::convert::From<&'a str> for ColumnRef<'a> {
    #[inline]
    fn from(val: &'a str) -> Self {
        ColumnRef::Column(val.into())
    }
}

impl<'a> std::convert::From<(&'a str, &'a str)> for ColumnRef<'a> {
    #[inline]
    fn from(val: (&'a str, &'a str)) -> Self {
        ColumnRef::TableColumn(val.0.into(), val.1.into())
    }
}

impl<'a> std::convert::From<(&'a str, &'a str, &'a str)> for ColumnRef<'a> {
    #[inline]
    fn from(val: (&'a str, &'a str, &'a str)) -> Self {
        ColumnRef::SchemaTableColumn(val.0.into(), val.1.into(), val.2.into())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableRef<'a> {
    Table(Ident<'a>),
    SchemaTable(Ident<'a>, Ident<'a>),
}

crate::macros::gen_display!(TableRef<'_>);

impl<'a> std::convert::From<&'a str> for TableRef<'a> {
    #[inline]
    fn from(val: &'a str) -> Self {
        TableRef::Table(val.into())
    }
}

impl<'a> std::convert::From<(&'a str, &'a str)> for TableRef<'a> {
    #[inline]
    fn from(val: (&'a str, &'a str)) -> Self {
        TableRef::SchemaTable(val.0.into(), val.1.into())
    }
}

impl std::default::Default for TableRef<'_> {
    #[inline]
    fn default() -> Self {
        TableRef::Table(Default::default())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sort {
    Asc,
    Desc,
}

impl Default for Sort {
    #[inline]
    fn default() -> Self {
        Sort::Asc
    }
}

crate::macros::gen_display!(Sort);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Order<'a>(pub(crate) Expr<'a>, pub(crate) Option<Sort>);

crate::macros::gen_display!(Order<'_>);

impl<'a, E> std::convert::From<(E, Sort)> for Order<'a>
where
    E: Into<Expr<'a>>,
{
    #[inline]
    fn from(val: (E, Sort)) -> Self {
        Order(val.0.into(), Some(val.1))
    }
}

impl<'a, E> std::convert::From<(E, Option<Sort>)> for Order<'a>
where
    E: Into<Expr<'a>>,
{
    #[inline]
    fn from(val: (E, Option<Sort>)) -> Self {
        Order(val.0.into(), val.1)
    }
}

impl<'a, E> std::convert::From<E> for Order<'a>
where
    E: Into<Expr<'a>>,
{
    #[inline]
    fn from(val: E) -> Self {
        Order(val.into(), None)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FuncCall<'a>(pub(crate) FuncRef<'a>, pub(crate) Vec<Expr<'a>>);

crate::macros::gen_display!(FuncCall<'_>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuncRef<'a> {
    Func(Ident<'a>),
    SchemaFunc(Ident<'a>, Ident<'a>),
}

crate::macros::gen_display!(FuncRef<'_>);

impl std::default::Default for FuncRef<'_> {
    #[inline]
    fn default() -> Self {
        FuncRef::Func(Default::default())
    }
}

impl<'a> std::convert::From<&'a str> for FuncRef<'a> {
    #[inline]
    fn from(val: &'a str) -> Self {
        FuncRef::Func(val.into())
    }
}

impl<'a> std::convert::From<(&'a str, &'a str)> for FuncRef<'a> {
    #[inline]
    fn from(val: (&'a str, &'a str)) -> Self {
        FuncRef::SchemaFunc(val.0.into(), val.1.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field<'a> {
    pub expr: Expr<'a>,
    pub alias: Option<Ident<'a>>,
}

crate::macros::gen_display!(Field<'_>);

impl<'a, E> std::convert::From<E> for Field<'a>
where
    E: Into<Expr<'a>>,
{
    #[inline]
    fn from(val: E) -> Self {
        Field {
            expr: val.into(),
            alias: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Table<'a> {
    pub table: TableExpr<'a>,
    pub alias: Option<Ident<'a>>,
}

crate::macros::gen_display!(Table<'_>);

impl<'a, T> std::convert::From<T> for Table<'a>
where
    T: Into<TableExpr<'a>>,
{
    #[inline]
    fn from(val: T) -> Self {
        Table {
            table: val.into(),
            alias: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Row<'a>(pub(crate) Vec<Expr<'a>>);

crate::macros::gen_display!(Row<'_>);
crate::macros::gen_impl_from_arr!(Row[Expr]<'a>);
crate::macros::gen_impl_from_vec!(Row[Expr]<'a>);
crate::macros::gen_impl_from_tup!(Row[Expr]<'a>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cte<'a> {
    pub(crate) name: Ident<'a>,
    pub(crate) columns: Vec<Ident<'a>>,
    pub(crate) stmt: crate::stmt::Stmt<'a>,
}

crate::macros::gen_display!(Cte<'_>);
