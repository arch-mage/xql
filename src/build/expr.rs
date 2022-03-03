use super::Dialect;
use super::ToSql;

impl<'a> ToSql<'a> for crate::expr::Expr<'a> {
    #[inline]
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        match self {
            crate::expr::Expr::Literal(val) => val.build::<D>(sql, args),
            crate::expr::Expr::Column(val) => val.build::<D>(sql, args),
            crate::expr::Expr::FuncCall(val) => val.build::<D>(sql, args),
            crate::expr::Expr::Prefix(op, val) => {
                sql.push_str(op);
                sql.push(' ');
                val.build::<D>(sql, args);
            }
            crate::expr::Expr::Infix(left, op, right) => {
                left.build::<D>(sql, args);
                sql.push(' ');
                sql.push_str(op);
                sql.push(' ');
                right.build::<D>(sql, args);
            }
            crate::expr::Expr::Postfix(val, op) => {
                val.build::<D>(sql, args);
                sql.push(' ');
                sql.push_str(op);
            }
            crate::expr::Expr::Paren(val) => {
                sql.push('(');
                val.build::<D>(sql, args);
                sql.push(')');
            }
            crate::expr::Expr::SubQuery(val) => {
                sql.push('(');
                val.build::<D>(sql, args);
                sql.push(')');
            }
        }
    }
}
