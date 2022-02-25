use crate::expr::Expr;
use crate::item::FuncCall;
use crate::item::FuncRef;

/// Construct a function call expression.
#[inline]
pub fn func<'a, F, A, I>(func: F, args: I) -> FuncCall<'a>
where
    F: Into<FuncRef<'a>>,
    A: Into<Expr<'a>>,
    I: IntoIterator<Item = A>,
{
    FuncCall(func.into(), args.into_iter().map(Into::into).collect())
}

macro_rules! gen_funcs {
    ($($(#[$comment:meta])* $func:ident),+) => {
        $(
            $(#[$comment])*
            #[inline]
            pub fn $func<'a, A>(arg: A) -> FuncCall<'a>
            where
                A: Into<Expr<'a>>,
            {
                func(stringify!($func), [arg.into()])
            }
        )+
    };
}

gen_funcs!(
    /// Construct a call to `sum` aggregate function.
    sum,
    /// Construct a call to `count` aggregate function.
    count,
    /// Construct a call to `avg` aggregate function.
    avg,
    /// Construct a call to `min` aggregate function.
    min,
    /// Construct a call to `max` aggregate function.
    max
);
