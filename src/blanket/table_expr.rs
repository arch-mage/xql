use crate::table_expr::TableExpr;

macro_rules! gen_method {
    ($join:ident cond) => {
        #[doc = concat!("A short hand for [`xql::ops::", stringify!($join), "`]", "(crate::ops::", stringify!($join), ").")]
        #[doc = ""]
        #[doc = "```"]
        #[doc = "use xql::ops::eq;"]
        #[doc = concat!("use xql::ops::", stringify!($join), ";")]
        #[doc = concat!("use xql::blanket::TableExprExt;")]
        #[doc = ""]
        #[doc = "assert_eq!("]
        #[doc = concat!(r#"    "category"."#, stringify!($join), r#"("book", eq(("category", "id"), ("book", "category_id"))),"#)]
        #[doc = concat!(r#"    "#, stringify!($join), r#"("category", "book", eq(("category", "id"), ("book", "category_id"))),"#)]
        #[doc = ");"]
        #[doc = "```"]
        #[inline]
        fn $join<R, E>(self, right: R, cond: E) -> $crate::table_expr::TableExpr<'a>
        where
            R: Into<$crate::table_expr::TableExpr<'a>>,
            E: Into<$crate::expr::Expr<'a>>,
        {
            $crate::ops::$join(self, right, cond)
        }
    };

    ($join:ident) => {
        #[doc = concat!("A short hand for [`xql::ops::", stringify!($join), "`]", "(crate::ops::", stringify!($join), ").")]
        #[doc = ""]
        #[doc = "```"]
        #[doc = "use xql::ops::eq;"]
        #[doc = concat!("use xql::ops::", stringify!($join), ";")]
        #[doc = concat!("use xql::blanket::TableExprExt;")]
        #[doc = ""]
        #[doc = "assert_eq!("]
        #[doc = concat!(r#"    "category"."#, stringify!($join), r#"("book"),"#)]
        #[doc = concat!(r#"    "#, stringify!($join), r#"("category", "book"),"#)]
        #[doc = ");"]
        #[doc = "```"]
        #[inline]
        fn $join<R>(self, right: R) -> $crate::table_expr::TableExpr<'a>
        where
            R: Into<$crate::table_expr::TableExpr<'a>>,
        {
            $crate::ops::$join(self, right)
        }
    };
}

/// Extends anything that can be converted to
/// [`TableExpr`](crate::table_expr::TableExpr) with it's related functions.
pub trait TableExprExt<'a>: Into<crate::table_expr::TableExpr<'a>> {
    gen_method!(join cond);
    gen_method!(left_join cond);
    gen_method!(right_join cond);
    gen_method!(full_join cond);
    gen_method!(cross_join);
    gen_method!(natural_join);
    gen_method!(natural_left_join);
    gen_method!(natural_right_join);
    gen_method!(natural_full_join);
}

impl<'a, T> TableExprExt<'a> for T where T: Into<TableExpr<'a>> {}

#[test]
#[cfg(test)]
fn test() {
    use crate::ops::*;

    assert_eq!(
        "category".join("book", eq(("category", "id"), ("book", "category_id"))),
        join(
            "category",
            "book",
            eq(("category", "id"), ("book", "category_id"))
        ),
    );
    assert_eq!(
        "category".left_join("book", eq(("category", "id"), ("book", "category_id"))),
        left_join(
            "category",
            "book",
            eq(("category", "id"), ("book", "category_id"))
        ),
    );
    assert_eq!(
        "category".right_join("book", eq(("category", "id"), ("book", "category_id"))),
        right_join(
            "category",
            "book",
            eq(("category", "id"), ("book", "category_id"))
        ),
    );
    assert_eq!(
        "category".full_join("book", eq(("category", "id"), ("book", "category_id"))),
        full_join(
            "category",
            "book",
            eq(("category", "id"), ("book", "category_id"))
        ),
    );

    assert_eq!(
        "category".cross_join("book"),
        cross_join("category", "book")
    );
    assert_eq!(
        "category".natural_join("book"),
        natural_join("category", "book")
    );
    assert_eq!(
        "category".natural_left_join("book"),
        natural_left_join("category", "book")
    );
    assert_eq!(
        "category".natural_right_join("book"),
        natural_right_join("category", "book")
    );
    assert_eq!(
        "category".natural_full_join("book"),
        natural_full_join("category", "book")
    );
}
