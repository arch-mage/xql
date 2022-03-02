use super::Build;
use super::Dialect;

impl<'a> Build<'a> for crate::table_expr::TableExpr<'a> {
    #[inline]
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        match self {
            crate::table_expr::TableExpr::TableRef(val) => val.build::<D>(sql, args),
            crate::table_expr::TableExpr::FuncCall(val) => val.build::<D>(sql, args),
            crate::table_expr::TableExpr::Join(left, right, cond) => {
                join!(D, sql, args, left, "JOIN", right, cond)
            }
            crate::table_expr::TableExpr::LeftJoin(left, right, cond) => {
                join!(D, sql, args, left, "LEFT JOIN", right, cond)
            }
            crate::table_expr::TableExpr::RightJoin(left, right, cond) => {
                join!(D, sql, args, left, "RIGHT JOIN", right, cond)
            }
            crate::table_expr::TableExpr::FullJoin(left, right, cond) => {
                join!(D, sql, args, left, "FULL JOIN", right, cond)
            }
            crate::table_expr::TableExpr::NaturalJoin(left, right) => {
                join!(D, sql, args, left, "NATURAL JOIN", right)
            }
            crate::table_expr::TableExpr::NaturalLeftJoin(left, right) => {
                join!(D, sql, args, left, "NATURAL LEFT JOIN", right)
            }
            crate::table_expr::TableExpr::NaturalRightJoin(left, right) => {
                join!(D, sql, args, left, "NATURAL RIGHT JOIN", right)
            }
            crate::table_expr::TableExpr::NaturalFullJoin(left, right) => {
                join!(D, sql, args, left, "NATURAL FULL JOIN", right)
            }
            crate::table_expr::TableExpr::CrossJoin(left, right) => {
                join!(D, sql, args, left, "CROSS JOIN", right)
            }
            crate::table_expr::TableExpr::SubQuery(val) => {
                sql.push('(');
                val.build::<D>(sql, args);
                sql.push(')');
            }
        }
    }
}
