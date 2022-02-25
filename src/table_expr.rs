use crate::expr::Expr;
use crate::item::FuncCall;
use crate::item::TableRef;

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
}

impl std::fmt::Display for TableExpr<'_> {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TableExpr::TableRef(val) => write!(f, "{val}"),
            TableExpr::FuncCall(val) => write!(f, "{val}"),
            TableExpr::Join(left, right, cond) => write!(f, "{left} JOIN {right} ON {cond}"),
            TableExpr::LeftJoin(left, right, cond) => write!(f, "{left} LEFT JOIN {right} ON {cond}"),
            TableExpr::RightJoin(left, right, cond) => write!(f, "{left} RIGHT JOIN {right} ON {cond}"),
            TableExpr::FullJoin(left, right, cond) => write!(f, "{left} FULL JOIN {right} ON {cond}"),
            TableExpr::NaturalJoin(left, right) => write!(f, "{left} NATURAL JOIN {right}"),
            TableExpr::NaturalLeftJoin(left, right) => write!(f, "{left} NATURAL LEFT JOIN {right}"),
            TableExpr::NaturalRightJoin(left, right) => write!(f, "{left} NATURAL RIGHT JOIN {right}"),
            TableExpr::NaturalFullJoin(left, right) => write!(f, "{left} NATURAL FULL JOIN {right}"),
            TableExpr::CrossJoin(left, right) => write!(f, "{left} CROSS JOIN {right}"),
        }
    }
}

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

#[test]
#[cfg(test)]
fn test() {
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
}
