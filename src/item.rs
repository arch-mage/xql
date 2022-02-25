use crate::expr::Expr;
use crate::table_expr::TableExpr;
use crate::utils::join;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ident<'a>(pub(crate) &'a str);

impl std::fmt::Display for Ident<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", crate::utils::quote_pretty(self.0, '"'))
    }
}

impl<'a> std::convert::From<&'a str> for Ident<'a> {
    #[inline]
    fn from(val: &'a str) -> Self {
        Ident(val)
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnRef<'a> {
    Column(Ident<'a>),
    TableColumn(Ident<'a>, Ident<'a>),
    SchemaTableColumn(Ident<'a>, Ident<'a>, Ident<'a>),
}

impl std::fmt::Display for ColumnRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ColumnRef::Column(col) => write!(f, "{}", col),
            ColumnRef::TableColumn(tbl, col) => write!(f, "{}.{}", tbl, col),
            ColumnRef::SchemaTableColumn(sch, tbl, col) => write!(f, "{}.{}.{}", sch, tbl, col),
        }
    }
}

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

impl std::fmt::Display for TableRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TableRef::Table(val) => write!(f, "{val}"),
            TableRef::SchemaTable(sch, tbl) => write!(f, "{sch}.{tbl}"),
        }
    }
}

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

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Sort::Asc => f.write_str("ASC"),
            Sort::Desc => f.write_str("DESC"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Order<'a>(pub(crate) Expr<'a>, pub(crate) Option<Sort>);

impl std::fmt::Display for Order<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Order(ord, None) => write!(f, "{}", ord),
            Order(ord, Some(sort)) => write!(f, "{} {}", ord, sort),
        }
    }
}

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

impl std::fmt::Display for FuncCall<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.0, join(&self.1, ", "))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuncRef<'a> {
    Func(Ident<'a>),
    SchemaFunc(Ident<'a>, Ident<'a>),
}

impl std::fmt::Display for FuncRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FuncRef::Func(val) => write!(f, "{val}"),
            FuncRef::SchemaFunc(sch, func) => write!(f, "{sch}.{func}"),
        }
    }
}

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

impl std::fmt::Display for Field<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.expr)?;
        if let Some(alias) = &self.alias {
            write!(f, " AS {alias}")?;
        }
        Ok(())
    }
}

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

impl std::fmt::Display for Table<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.table)?;
        if let Some(alias) = &self.alias {
            write!(f, " AS {alias}")?;
        }
        Ok(())
    }
}

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

impl std::fmt::Display for Row<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})", join(&self.0, ", "))
    }
}

crate::macros::gen_impl_from_arr!(Row[Expr]<'a>);
crate::macros::gen_impl_from_vec!(Row[Expr]<'a>);
crate::macros::gen_impl_from_tup!(Row[Expr]<'a>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cte<'a> {
    pub(crate) name: Ident<'a>,
    pub(crate) columns: Vec<Ident<'a>>,
    pub(crate) stmt: crate::stmt::Stmt<'a>,
}

impl std::fmt::Display for Cte<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.columns.is_empty() {
            write!(f, "({})", join(&self.columns, ", "))?;
        }
        write!(f, " AS ({})", self.stmt)
    }
}
