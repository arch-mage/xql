use crate::clause;
use crate::expr::Expr;
use crate::stmt::result::Result;

/// `VALUES` statement builder.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Values<'a> {
    pub(crate) with: Option<clause::With<'a>>,
    pub(crate) rows: clause::Values<'a>,
}

stmt_common!(Values);

impl std::fmt::Display for Values<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write_option!(with: self, f, with)?;
        write!(f, "{}", self.rows)
    }
}

impl<'a> Values<'a> {
    pub fn limit<E>(self, expr: E) -> Result<'a>
    where
        E: Into<Expr<'a>>,
    {
        Result {
            data: self.into(),
            limit: Some(clause::Limit(expr.into())),
            ..Default::default()
        }
    }

    pub fn offset<E>(self, expr: E) -> Result<'a>
    where
        E: Into<Expr<'a>>,
    {
        Result {
            data: self.into(),
            offset: Some(clause::Offset(expr.into())),
            ..Default::default()
        }
    }
}
