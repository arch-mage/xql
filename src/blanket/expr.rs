use crate::expr::Expr;
use crate::item::Field;
use crate::item::Ident;
use crate::item::Order;

macro_rules! gen_method {
    ($method:ident) => {
        #[doc = concat!("A short hand for [`xql::ops::", stringify!($method), "`]", "(crate::ops::", stringify!($method), ").")]
        #[doc = ""]
        #[doc = "```"]
        #[doc = concat!("use xql::ops::", stringify!($method), ";")]
        #[doc = concat!("use xql::blanket::ExprExt;")]
        #[doc = ""]
        #[doc = concat!("assert_eq!(1.", stringify!($method), "(2), ", stringify!($method), "(1, 2));")]
        #[doc = "```"]
        #[inline]
        fn $method<R: Into<$crate::expr::Expr<'a>>>(self, right: R) -> $crate::expr::Expr<'a> {
            $crate::ops::$method(self, right)
        }
    };

    ($method:ident as $alias:ident) => {
        #[doc = concat!("An alias for [`xql::ops::", stringify!($method), "`]", "(crate::blanket::ExprExt::", stringify!($method), ").")]
        #[doc = ""]
        #[doc = "```"]
        #[doc = concat!("use xql::ops::", stringify!($method), ";")]
        #[doc = concat!("use xql::blanket::ExprExt;")]
        #[doc = ""]
        #[doc = concat!("assert_eq!(1.", stringify!($alias), "(2), ", stringify!($method), "(1, 2));")]
        #[doc = "```"]
        #[inline]
        fn $alias<R: Into<$crate::expr::Expr<'a>>>(self, right: R) -> $crate::expr::Expr<'a> {
            $crate::ops::$method(self, right)
        }
    };
}

pub trait ExprExt<'a>: Sized + Into<Expr<'a>> {
    gen_method!(eq);
    gen_method!(eq as equal);

    gen_method!(ne);
    gen_method!(ne as not_equal);

    gen_method!(gt);
    gen_method!(gt as greater_than);

    gen_method!(ge);
    gen_method!(ge as greater_equal);

    gen_method!(lt);
    gen_method!(lt as less_than);

    gen_method!(le);
    gen_method!(le as less_equal);

    gen_method!(and);
    gen_method!(or);
    gen_method!(like);
    gen_method!(ilike);

    /// A short hand for [`xql::ops::as_field`](crate::ops::as_field).
    ///
    /// ```
    /// use xql::ops::as_field;
    /// use xql::blanket::ExprExt;
    ///
    /// assert_eq!(1.alias("id"), as_field(1, "id"));
    /// ```
    #[inline]
    fn alias<A: Into<Ident<'a>>>(self, alias: A) -> Field<'a> {
        crate::ops::as_field(self, alias)
    }

    /// A short hand for [`xql::ops::isnull`](crate::ops::isnull).
    ///
    /// ```
    /// use xql::ops::isnull;
    /// use xql::blanket::ExprExt;
    ///
    /// assert_eq!(1.isnull(), isnull(1));
    /// ```
    #[inline]
    fn isnull(self) -> Expr<'a> {
        crate::ops::isnull(self)
    }

    /// A short hand for [`xql::ops::paren`](crate::ops::paren).
    ///
    /// ```
    /// use xql::ops::paren;
    /// use xql::blanket::ExprExt;
    ///
    /// assert_eq!(1.paren(), paren(1));
    /// ```
    #[inline]
    fn paren(self) -> Expr<'a> {
        crate::ops::paren(self)
    }

    /// A short hand for [`xql::ops::asc`](crate::ops::asc).
    ///
    /// ```
    /// use xql::ops::asc;
    /// use xql::blanket::ExprExt;
    ///
    /// assert_eq!("id".asc(), asc("id"));
    /// ```
    #[inline]
    fn asc(self) -> Order<'a> {
        crate::ops::asc(self)
    }
}

impl<'a, T> ExprExt<'a> for T where T: Into<Expr<'a>> {}

#[test]
#[cfg(test)]
fn binops() {
    assert_eq!(1.eq(1).to_string(), "1 = 1");
    assert_eq!(1.equal(1).to_string(), "1 = 1");
    assert_eq!(1.ne(1).to_string(), "1 <> 1");
    assert_eq!(1.not_equal(1).to_string(), "1 <> 1");
    assert_eq!(1.gt(1).to_string(), "1 > 1");
    assert_eq!(1.greater_than(1).to_string(), "1 > 1");
    assert_eq!(1.ge(1).to_string(), "1 >= 1");
    assert_eq!(1.greater_equal(1).to_string(), "1 >= 1");
    assert_eq!(1.lt(1).to_string(), "1 < 1");
    assert_eq!(1.less_than(1).to_string(), "1 < 1");
    assert_eq!(1.le(1).to_string(), "1 <= 1");
    assert_eq!(1.less_equal(1).to_string(), "1 <= 1");

    assert_eq!(1.and(1).to_string(), "1 AND 1");
    assert_eq!(1.or(1).to_string(), "1 OR 1");
    assert_eq!(1.like(1).to_string(), "1 LIKE 1");
    assert_eq!(1.ilike(1).to_string(), "1 ILIKE 1");
}
