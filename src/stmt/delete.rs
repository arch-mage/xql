use crate::clause;
use crate::expr::Expr;
use crate::ops::and;

/// `DELETE` statement builder.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Delete<'a> {
    pub(crate) with: Option<clause::With<'a>>,
    pub(crate) table: clause::Delete<'a>,
    pub(crate) filter: Option<clause::Where<'a>>,
    pub(crate) returns: Option<clause::Returning<'a>>,
}

stmt_common!(Delete);

crate::macros::gen_display!(Delete<'_>);

impl<'a> Delete<'a> {
    /// Set condition to `WHERE` clause.
    ///
    /// Successive calls combine new condition with previous condition with
    /// [`and`](crate::ops::and).
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::delete;
    /// use xql::and;
    /// use xql::ge;
    ///
    /// let query1 = delete("book")
    ///     .filter(and(ge("id", 1), ge("year", 1970)));
    ///
    /// let query2 = delete("book")
    ///     .filter(ge("id", 1))
    ///     .filter(ge("year", 1970));
    ///
    /// assert_eq!(query1, query2);
    /// ```
    pub fn filter<E>(mut self, expr: E) -> Delete<'a>
    where
        E: Into<Expr<'a>>,
    {
        self.filter = match self.filter.take() {
            Some(inner) => Some(and(inner.0, expr.into()).into()),
            None => Some(expr.into().into()),
        };
        self
    }

    /// Set/Add field(s) to `RETURNING` clause.
    ///
    /// Successive calls combine adds more field into the clause.
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::delete;
    ///
    /// let query1 = delete("book")
    ///     .returning(["id", "name"]);
    ///
    /// let query2 = delete("book")
    ///     .returning(["id"])
    ///     .returning(["name"]);
    ///
    /// assert_eq!(query1, query2);
    /// ```
    pub fn returning<T>(mut self, returns: T) -> Delete<'a>
    where
        T: Into<clause::Returning<'a>>,
    {
        self.returns = match self.returns.take() {
            Some(mut inner) => {
                inner.0.extend(returns.into().0);
                Some(inner)
            }
            None => Some(returns.into()),
        };
        self
    }
}

#[test]
#[cfg(test)]
fn test() {
    use crate::ops;

    let query = crate::delete("user")
        .filter(and(
            ops::not(("user", "active")),
            ops::isnull(("user", "name")),
        ))
        .returning(["id", "name"]);
    assert_eq!(
        query.to_string(),
        "DELETE FROM user WHERE NOT user.active AND user.name ISNULL RETURNING id, name"
    );
}
