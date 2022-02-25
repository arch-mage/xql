use crate::expr::Expr;
use crate::table_expr::TableExpr;
use crate::item::Field;
use crate::item::Table;
use crate::item::Ident;
use crate::item::Order;
use crate::item::Sort;

/// Make an alias out of an expression.
///
/// # Examples
///
/// ```
/// use xql::as_field;
///
/// assert_eq!(as_field(1, "num").to_string(), "1 AS num");
/// ```
#[inline]
pub fn as_field<'a, E, A>(expr: E, alias: A) -> Field<'a>
where
    E: Into<Expr<'a>>,
    A: Into<Ident<'a>>,
{
    Field {
        expr: expr.into(),
        alias: Some(alias.into()),
    }
}

/// Make an alias out of a table expression.
///
/// # Examples
///
/// ```
/// use xql::as_table;
///
/// assert_eq!(as_table("user", "person").to_string(), "user AS person");
/// ```
#[inline]
pub fn as_table<'a, T, A>(table: T, alias: A) -> Table<'a>
where
    T: Into<TableExpr<'a>>,
    A: Into<Ident<'a>>,
{
    Table {
        table: table.into(),
        alias: Some(alias.into()),
    }
}

/// Group an expression with parenthesis.
///
/// # Examples
///
/// ```
/// use xql::{paren, not, and};
///
/// assert_eq!(not(paren(and(true, false))).to_string(), "NOT (true AND false)");
/// ```
#[inline]
pub fn paren<'a, E>(expr: E) -> Expr<'a>
where
    E: Into<Expr<'a>>,
{
    Expr::Paren(Box::new(expr.into()))
}

/// Make an ascending sort out of an expression.
///
/// # Examples
///
/// ```
/// use xql::asc;
///
/// assert_eq!(asc("id").to_string(), "id ASC");
/// assert_eq!(asc(("user", "id")).to_string(), "user.id ASC");
/// ```
#[inline]
pub fn asc<'a, E: Into<Expr<'a>>>(expr: E) -> Order<'a> {
    Order(expr.into(), Some(Sort::Asc))
}

/// Make a descending sort out of an expression.
///
/// # Examples
///
/// ```
/// use xql::desc;
///
/// assert_eq!(desc("id").to_string(), "id DESC");
/// assert_eq!(desc(("user", "id")).to_string(), "user.id DESC");
/// ```
#[inline]
pub fn desc<'a, E: Into<Expr<'a>>>(expr: E) -> Order<'a> {
    Order(expr.into(), Some(Sort::Desc))
}

/// Construct a binary operation on expression.
///
/// # Examples
///
/// ```
/// use xql::binop;
///
/// assert_eq!(binop(1, "+", 2).to_string(), "1 + 2");
/// ```
#[inline]
pub fn binop<'a, L, R>(left: L, op: &'static str, right: R) -> Expr<'a>
where
    L: Into<Expr<'a>>,
    R: Into<Expr<'a>>,
{
    Expr::Infix(Box::new(left.into()), op, Box::new(right.into()))
}

#[inline]
/// Construct a unary prefix operation on expression.
///
/// # Examples
///
/// ```
/// use xql::preop;
///
/// assert_eq!(preop("NOT", true).to_string(), "NOT true");
/// ```
pub fn preop<'a, E>(op: &'static str, expr: E) -> Expr<'a>
where
    E: Into<Expr<'a>>,
{
    Expr::Prefix(op, Box::new(expr.into()))
}

#[inline]
/// Construct a unary postfix operation on expression.
///
/// # Examples
///
/// ```
/// use xql::postop;
///
/// assert_eq!(postop("id", "ISNULL").to_string(), "id ISNULL");
/// ```
pub fn postop<'a, E>(expr: E, op: &'static str) -> Expr<'a>
where
    E: Into<Expr<'a>>,
{
    Expr::Postfix(Box::new(expr.into()), op)
}

macro_rules! generate_binop_funcs {
    ($trait:ident {$($(#[$comment:meta])* $name:ident: $op:expr),+}) => {
        $(
            $(#[$comment])*
            #[inline]
            pub fn $name<'a, L, R>(left: L, right: R) -> $crate::expr::Expr<'a>
            where L: Into<$crate::expr::Expr<'a>>,
                  R: Into<$crate::expr::Expr<'a>>,
            {
                crate::expr::Expr::Infix(
                    Box::new(left.into()),
                    $op,
                    Box::new(right.into()),
                )
            }
        )+
    };
}

generate_binop_funcs!(BinOps {
    /// Construct an `addition` operation between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::add;
    ///
    /// assert_eq!(add(1, 2).to_string(), "1 + 2");
    /// ```
    add: "+",
    /// Construct a `multiplication` operation between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::sub;
    ///
    /// assert_eq!(sub(1, 2).to_string(), "1 - 2");
    /// ```
    sub: "-",
    /// Construct a `division` operation between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::mul;
    ///
    /// assert_eq!(mul(1, 2).to_string(), "1 * 2");
    /// ```
    mul: "*",
    /// Construct a `modulo` operation between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::div;
    ///
    /// assert_eq!(div(1, 2).to_string(), "1 / 2");
    /// ```
    div: "/",
    /// Construct an `equal` comparison between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::rem;
    ///
    /// assert_eq!(rem(1, 2).to_string(), "1 % 2");
    /// ```
    rem: "%",
    /// Construct a `not equal` comparison between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::eq;
    ///
    /// assert_eq!(eq(1, 2).to_string(), "1 = 2");
    /// ```
    eq: "=",
    /// Construct a `greater than` comparison between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::ne;
    ///
    /// assert_eq!(ne(1, 2).to_string(), "1 <> 2");
    /// ```
    ne: "<>",
    /// Construct a `greater or equal` comparison between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::gt;
    ///
    /// assert_eq!(gt(1, 2).to_string(), "1 > 2");
    /// ```
    gt: ">",
    /// Construct a `less than` comparison between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::ge;
    ///
    /// assert_eq!(ge(1, 2).to_string(), "1 >= 2");
    /// ```
    ge: ">=",
    /// Construct a `less or equal` comparison between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::lt;
    ///
    /// assert_eq!(lt(1, 2).to_string(), "1 < 2");
    /// ```
    lt: "<",
    /// Construct a `less or equal` comparison between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::le;
    ///
    /// assert_eq!(le(1, 2).to_string(), "1 <= 2");
    /// ```
    le: "<=",
    /// Construct a `boolean and` operation between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::and;
    ///
    /// assert_eq!(and(true, false).to_string(), "true AND false");
    /// ```
    and: "AND",
    /// Construct a `boolean or` operation between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::or;
    ///
    /// assert_eq!(or(true, false).to_string(), "true OR false");
    /// ```
    or: "OR",
    /// Construct a `LIKE` operation between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::like;
    ///
    /// assert_eq!(like("name", &"%name".to_string()).to_string(), "name LIKE '%name'");
    /// ```
    like: "LIKE",
    /// Construct a `ILIKE` operation between two expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::ilike;
    ///
    /// assert_eq!(ilike("name", &"%name".to_string()).to_string(), "name ILIKE '%name'");
    /// ```
    ilike: "ILIKE"
});

/// Construct a `boolean not` operation on an expression.
///
/// # Examples
///
/// ```
/// use xql::not;
///
/// assert_eq!(not(true).to_string(), "NOT true");
/// ```
#[inline]
pub fn not<'a, E>(expr: E) -> Expr<'a>
where
    E: Into<Expr<'a>>,
{
    preop("NOT", expr)
}

/// Construct an `ISNULL` operation on an expression.
///
/// # Examples
///
/// ```
/// use xql::isnull;
///
/// assert_eq!(isnull(None::<i32>).to_string(), "null ISNULL");
/// ```
#[inline]
pub fn isnull<'a, E>(expr: E) -> Expr<'a>
where
    E: Into<Expr<'a>>,
{
    postop(expr, "ISNULL")
}

macro_rules! generate_join_funcs {
    ($(#[$comment:meta])* $join:ident $fn:ident) => {
        $(#[$comment])*
        #[inline]
        pub fn $fn<'a, L, R>(left: L, right: R) -> crate::table_expr::TableExpr<'a>
        where
            L: Into<crate::table_expr::TableExpr<'a>>,
            R: Into<crate::table_expr::TableExpr<'a>>,
        {
            crate::table_expr::TableExpr::$join(Box::new(left.into()), Box::new(right.into()))
        }
    };
    ($(#[$comment:meta])* $join:ident $fn:ident cond) => {
        $(#[$comment])*
        #[inline]
        pub fn $fn<'a, L, R, E>(left: L, right: R, cond: E) -> crate::table_expr::TableExpr<'a>
        where
            L: Into<crate::table_expr::TableExpr<'a>>,
            R: Into<crate::table_expr::TableExpr<'a>>,
            E: Into<crate::expr::Expr<'a>>,
        {
            crate::table_expr::TableExpr::$join(
                Box::new(left.into()),
                Box::new(right.into()),
                cond.into(),
            )
        }
    };
}

generate_join_funcs!(
    /// Construct a `JOIN` operation on a table expression.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::eq;
    /// use xql::join;
    /// 
    /// assert_eq!(
    ///     join("category", "book", eq(("category", "id"), ("book", "category_id"))).to_string(),
    ///     "category JOIN book ON category.id = book.category_id",
    /// );
    /// ```
    Join join cond);
generate_join_funcs!(
    /// Construct a `LEFT JOIN` operation on a table expression.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::eq;
    /// use xql::left_join;
    /// 
    /// assert_eq!(
    ///     left_join("category", "book", eq(("category", "id"), ("book", "category_id"))).to_string(),
    ///     "category LEFT JOIN book ON category.id = book.category_id",
    /// );
    /// ```
    LeftJoin left_join cond);
generate_join_funcs!(
    /// Construct a `RIGHT JOIN` operation on a table expression.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::eq;
    /// use xql::right_join;
    /// 
    /// assert_eq!(
    ///     right_join("category", "book", eq(("category", "id"), ("book", "category_id"))).to_string(),
    ///     "category RIGHT JOIN book ON category.id = book.category_id",
    /// );
    /// ```
    RightJoin right_join cond);
generate_join_funcs!(
    /// Construct a `FULL JOIN` operation on a table expression.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::eq;
    /// use xql::full_join;
    /// 
    /// assert_eq!(
    ///     full_join("category", "book", eq(("category", "id"), ("book", "category_id"))).to_string(),
    ///     "category FULL JOIN book ON category.id = book.category_id",
    /// );
    /// ```
    FullJoin full_join cond);
generate_join_funcs!(
    /// Construct a `NATURAL JOIN` operation on a table expression.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::natural_join;
    /// 
    /// assert_eq!(
    ///     natural_join("category", "book").to_string(),
    ///     "category NATURAL JOIN book",
    /// );
    /// ```
    NaturalJoin natural_join);
generate_join_funcs!(
    /// Construct a `NATURAL LEFT JOIN` operation on a table expression.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::natural_left_join;
    /// 
    /// assert_eq!(
    ///     natural_left_join("category", "book").to_string(),
    ///     "category NATURAL LEFT JOIN book",
    /// );
    /// ```
    NaturalLeftJoin natural_left_join);
generate_join_funcs!(
    /// Construct a `NATURAL RIGHT JOIN` operation on a table expression.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::natural_right_join;
    /// 
    /// assert_eq!(
    ///     natural_right_join("category", "book").to_string(),
    ///     "category NATURAL RIGHT JOIN book",
    /// );
    /// ```
    NaturalRightJoin natural_right_join);
generate_join_funcs!(
    /// Construct a `NATURAL FULL JOIN` operation on a table expression.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::natural_full_join;
    /// 
    /// assert_eq!(
    ///     natural_full_join("category", "book").to_string(),
    ///     "category NATURAL FULL JOIN book",
    /// );
    /// ```
    NaturalFullJoin natural_full_join);
generate_join_funcs!(
    /// Construct a `CROSS JOIN` operation on a table expression.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::cross_join;
    /// 
    /// assert_eq!(
    ///     cross_join("category", "book").to_string(),
    ///     "category CROSS JOIN book",
    /// );
    /// ```
    CrossJoin cross_join);
