use crate::item::ColumnRef;
use crate::item::FuncCall;
use crate::value::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr<'a> {
    Column(ColumnRef<'a>),
    Literal(Value<'a>),
    FuncCall(FuncCall<'a>),

    Prefix(&'static str, Box<Expr<'a>>),
    Infix(Box<Expr<'a>>, &'static str, Box<Expr<'a>>),
    Postfix(Box<Expr<'a>>, &'static str),
    Paren(Box<Expr<'a>>),
}

impl std::fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Column(val) => write!(f, "{val}"),
            Expr::Literal(val) => write!(f, "{val}"),
            Expr::FuncCall(val) => write!(f, "{val}"),
            Expr::Infix(left, op, right) => write!(f, "{left} {op} {right}"),
            Expr::Prefix(op, expr) => write!(f, "{op} {expr}"),
            Expr::Postfix(expr, op) => write!(f, "{expr} {op}"),
            Expr::Paren(expr) => write!(f, "({expr})"),
        }
    }
}

impl<'a> std::convert::From<&'a str> for Expr<'a> {
    #[inline]
    fn from(val: &'a str) -> Self {
        Expr::Column(val.into())
    }
}

impl<'a> std::convert::From<(&'a str, &'a str)> for Expr<'a> {
    #[inline]
    fn from(val: (&'a str, &'a str)) -> Self {
        Expr::Column(val.into())
    }
}

impl<'a> std::convert::From<(&'a str, &'a str, &'a str)> for Expr<'a> {
    #[inline]
    fn from(val: (&'a str, &'a str, &'a str)) -> Self {
        Expr::Column(val.into())
    }
}

impl<'a, T> std::convert::From<T> for Expr<'a>
where
    T: Into<Value<'a>>,
{
    #[inline]
    fn from(val: T) -> Self {
        Expr::Literal(val.into())
    }
}

impl<'a> std::convert::From<FuncCall<'a>> for Expr<'a> {
    #[inline]
    fn from(val: FuncCall<'a>) -> Self {
        Expr::FuncCall(val)
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::Expr;
    use crate::func::avg;
    use crate::func::count;
    use crate::func::sum;
    use crate::item::ColumnRef;
    use crate::item::Ident;
    use crate::item::Order;
    use crate::item::Sort;
    use crate::ops::*;
    use crate::value::Value;

    #[test]
    fn literal() {
        assert_eq!(Into::<Expr>::into(true), Expr::Literal(Value::Bool(true)));
        assert_eq!(Into::<Expr>::into(false), Expr::Literal(Value::Bool(false)));
        assert_eq!(Into::<Expr>::into(8_i8), Expr::Literal(Value::TinyInt(8)));
        assert_eq!(
            Into::<Expr>::into(16_i16),
            Expr::Literal(Value::SmallInt(16))
        );
        assert_eq!(Into::<Expr>::into(32_i32), Expr::Literal(Value::Int(32)));
        assert_eq!(Into::<Expr>::into(64_i64), Expr::Literal(Value::BigInt(64)));
        assert_eq!(
            Into::<Expr>::into(&"text".to_string()),
            Expr::Literal(Value::Text("text"))
        );
    }

    #[test]
    fn literal_fmt() {
        assert_eq!(Into::<Expr>::into(true).to_string(), "true");
        assert_eq!(Into::<Expr>::into(false).to_string(), "false");
        assert_eq!(Into::<Expr>::into(8_i8).to_string(), "8");
        assert_eq!(Into::<Expr>::into(16_i16).to_string(), "16");
        assert_eq!(Into::<Expr>::into(32_i32).to_string(), "32");
        assert_eq!(Into::<Expr>::into(64_i64).to_string(), "64");
        assert_eq!(
            Into::<Expr>::into(&"text".to_string()).to_string(),
            "'text'"
        );
    }

    #[test]
    fn option() {
        assert_eq!(Into::<Expr>::into(None::<i32>).to_string(), "null");
        assert_eq!(Into::<Expr>::into(Some(1)).to_string(), "1");
    }

    #[test]
    fn column_ref() {
        assert_eq!(
            Into::<Expr>::into("id"),
            Expr::Column(ColumnRef::Column(Ident("id")))
        );
        assert_eq!(
            Into::<Expr>::into(("user", "id")),
            Expr::Column(ColumnRef::TableColumn(Ident("user"), Ident("id")))
        );
        assert_eq!(
            Into::<Expr>::into(("public", "user", "id")),
            Expr::Column(ColumnRef::SchemaTableColumn(
                Ident("public"),
                Ident("user"),
                Ident("id")
            ))
        );
    }

    #[test]
    fn func_call() {
        assert_eq!(sum("num").to_string(), "sum(num)");
        assert_eq!(count("id").to_string(), "count(id)");
        assert_eq!(avg("age").to_string(), "avg(age)");
    }

    #[test]
    fn column_ref_fmt() {
        assert_eq!(Into::<Expr>::into("id").to_string(), "id",);
        assert_eq!(Into::<Expr>::into(("user", "id")).to_string(), "user.id");
        assert_eq!(
            Into::<Expr>::into(("public", "user", "id")).to_string(),
            "public.user.id"
        );
    }

    #[test]
    #[rustfmt::skip]
    fn ops() {
        assert_eq!(add(1, 2),    Expr::Infix(Box::new(1.into()), "+",      Box::new(2.into())));
        assert_eq!(sub(1, 2),    Expr::Infix(Box::new(1.into()), "-",      Box::new(2.into())));
        assert_eq!(mul(1, 2),    Expr::Infix(Box::new(1.into()), "*",      Box::new(2.into())));
        assert_eq!(div(1, 2),    Expr::Infix(Box::new(1.into()), "/",      Box::new(2.into())));
        assert_eq!(rem(1, 2),    Expr::Infix(Box::new(1.into()), "%",      Box::new(2.into())));
        assert_eq!(eq(1, 2),     Expr::Infix(Box::new(1.into()), "=",      Box::new(2.into())));
        assert_eq!(ne(1, 2),     Expr::Infix(Box::new(1.into()), "<>",     Box::new(2.into())));
        assert_eq!(gt(1, 2),     Expr::Infix(Box::new(1.into()), ">",      Box::new(2.into())));
        assert_eq!(ge(1, 2),     Expr::Infix(Box::new(1.into()), ">=",     Box::new(2.into())));
        assert_eq!(lt(1, 2),     Expr::Infix(Box::new(1.into()), "<",      Box::new(2.into())));
        assert_eq!(le(1, 2),     Expr::Infix(Box::new(1.into()), "<=",     Box::new(2.into())));
        assert_eq!(and(1, 2),    Expr::Infix(Box::new(1.into()), "AND",    Box::new(2.into())));
        assert_eq!(or(1, 2),     Expr::Infix(Box::new(1.into()), "OR",     Box::new(2.into())));
        assert_eq!(like(1, 2),   Expr::Infix(Box::new(1.into()), "LIKE",   Box::new(2.into())));
        assert_eq!(ilike(1, 2),  Expr::Infix(Box::new(1.into()), "ILIKE",  Box::new(2.into())));

        assert_eq!(asc("id"),  Order(Expr::Column(ColumnRef::Column("id".into())), Some(Sort::Asc)));
        assert_eq!(desc("id"), Order(Expr::Column(ColumnRef::Column("id".into())), Some(Sort::Desc)));

        assert_eq!(not(true), Expr::Prefix("NOT", Box::new(true.into())));
        assert_eq!(isnull("expr"), Expr::Postfix(Box::new("expr".into()), "ISNULL"));
    }

    #[test]
    fn parenthesis() {
        let cond1 = or("a", "b");
        let cond2 = or("c", "d");
        let cond = and(paren(cond1), paren(cond2));
        assert_eq!(cond.to_string(), "(a OR b) AND (c OR d)");
    }
}
