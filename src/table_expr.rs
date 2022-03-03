use crate::expr::Expr;
use crate::item::FuncCall;
use crate::item::TableRef;
use crate::stmt::data::Data;
use crate::stmt::select::Select;
use crate::stmt::values::Values;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TableExpr<'a> {
    TableRef(TableRef<'a>),
    FuncCall(FuncCall<'a>),
    Join(Box<TableExpr<'a>>, Box<TableExpr<'a>>, Expr<'a>),
    LeftJoin(Box<TableExpr<'a>>, Box<TableExpr<'a>>, Expr<'a>),
    RightJoin(Box<TableExpr<'a>>, Box<TableExpr<'a>>, Expr<'a>),
    FullJoin(Box<TableExpr<'a>>, Box<TableExpr<'a>>, Expr<'a>),
    NaturalJoin(Box<TableExpr<'a>>, Box<TableExpr<'a>>),
    NaturalLeftJoin(Box<TableExpr<'a>>, Box<TableExpr<'a>>),
    NaturalRightJoin(Box<TableExpr<'a>>, Box<TableExpr<'a>>),
    NaturalFullJoin(Box<TableExpr<'a>>, Box<TableExpr<'a>>),
    CrossJoin(Box<TableExpr<'a>>, Box<TableExpr<'a>>),

    SubQuery(Data<'a>),
}

crate::macros::gen_display!(TableExpr<'_>);

impl<'a> std::convert::From<&'a str> for TableExpr<'a> {
    #[inline]
    fn from(val: &'a str) -> Self {
        TableExpr::TableRef(val.into())
    }
}

impl<'a> std::convert::From<(&'a str, &'a str)> for TableExpr<'a> {
    #[inline]
    fn from(val: (&'a str, &'a str)) -> Self {
        TableExpr::TableRef(val.into())
    }
}

impl<'a> std::convert::From<FuncCall<'a>> for TableExpr<'a> {
    #[inline]
    fn from(val: FuncCall<'a>) -> Self {
        TableExpr::FuncCall(val)
    }
}

impl<'a> std::convert::From<Data<'a>> for TableExpr<'a> {
    #[inline]
    fn from(val: Data<'a>) -> Self {
        TableExpr::SubQuery(val)
    }
}

impl<'a> std::convert::From<Select<'a>> for TableExpr<'a> {
    #[inline]
    fn from(val: Select<'a>) -> Self {
        TableExpr::SubQuery(val.into())
    }
}

impl<'a> std::convert::From<Values<'a>> for TableExpr<'a> {
    #[inline]
    fn from(val: Values<'a>) -> Self {
        TableExpr::SubQuery(val.into())
    }
}

#[test]
#[cfg(test)]
fn test() {
    use crate::ops::as_field;
    use crate::ops::as_table;
    use crate::ops::cross_join;
    use crate::ops::eq;
    use crate::ops::full_join;
    use crate::ops::join;
    use crate::ops::left_join;
    use crate::ops::natural_full_join;
    use crate::ops::natural_join;
    use crate::ops::natural_left_join;
    use crate::ops::natural_right_join;
    use crate::ops::right_join;
    use crate::stmt::select;

    let query = join("a", "b", eq(("a", "id"), ("b", "id")));
    assert_eq!(query.to_string(), "a JOIN b ON a.id = b.id");
    let query = left_join("a", "b", eq(("a", "id"), ("b", "id")));
    assert_eq!(query.to_string(), "a LEFT JOIN b ON a.id = b.id");
    let query = right_join("a", "b", eq(("a", "id"), ("b", "id")));
    assert_eq!(query.to_string(), "a RIGHT JOIN b ON a.id = b.id");
    let query = full_join("a", "b", eq(("a", "id"), ("b", "id")));
    assert_eq!(query.to_string(), "a FULL JOIN b ON a.id = b.id");

    let query = natural_join("a", "b");
    assert_eq!(query.to_string(), "a NATURAL JOIN b");
    let query = natural_left_join("a", "b");
    assert_eq!(query.to_string(), "a NATURAL LEFT JOIN b");
    let query = natural_right_join("a", "b");
    assert_eq!(query.to_string(), "a NATURAL RIGHT JOIN b");
    let query = natural_full_join("a", "b");
    assert_eq!(query.to_string(), "a NATURAL FULL JOIN b");

    let query = cross_join("a", "b");
    assert_eq!(query.to_string(), "a CROSS JOIN b");

    let query = select([("sub", "one"), ("sub", "two")]).from(as_table(
        select([as_field(1, "one"), as_field(2, "two")]),
        "sub",
    ));
    assert_eq!(
        query.to_string(),
        "SELECT sub.one, sub.two FROM (SELECT 1 AS one, 2 AS two) AS sub"
    );
}
