use crate::clause;
use crate::expr::Expr;
use crate::item::Ident;
use crate::ops::and;

/// `UPDATE` statement builder.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Update<'a> {
    pub(crate) with: Option<clause::With<'a>>,
    pub(crate) table: clause::Update<'a>,
    pub(crate) set: clause::Set<'a>,
    pub(crate) from: Option<clause::From<'a>>,
    pub(crate) filter: Option<clause::Where<'a>>,
    pub(crate) returns: Option<clause::Returning<'a>>,
}

stmt_common!(Update);

crate::macros::gen_display!(Update<'_>);

impl<'a> Update<'a> {
    pub fn set<C, V>(mut self, column: C, value: V) -> Update<'a>
    where
        C: Into<Ident<'a>>,
        V: Into<Expr<'a>>,
    {
        self.set.0.push((column.into(), value.into()));
        self
    }

    pub fn set_values<V>(mut self, values: V) -> Update<'a>
    where
        V: Into<clause::Set<'a>>,
    {
        self.set.0.extend(values.into().0);
        self
    }

    pub fn from<T>(mut self, tables: T) -> Update<'a>
    where
        T: Into<clause::From<'a>>,
    {
        self.from = match self.from.take() {
            Some(mut inner) => {
                inner.0.extend(tables.into().0);
                Some(inner)
            }
            None => Some(tables.into()),
        };
        self
    }

    /// Set condition to `WHERE` clause.
    ///
    /// Successive calls combine new condition with previous condition with
    /// [`and`](crate::ops::and).
    ///
    /// # Examples
    ///
    /// ```
    /// use xql::update;
    /// use xql::and;
    /// use xql::ge;
    ///
    /// let query1 = update("book")
    ///     .set("id", 1)
    ///     .filter(and(ge("id", 1), ge("year", 1970)));
    ///
    /// let query2 = update("book")
    ///     .set("id", 1)
    ///     .filter(ge("id", 1))
    ///     .filter(ge("year", 1970));
    ///
    /// assert_eq!(query1, query2);
    /// ```
    pub fn filter<E>(mut self, expr: E) -> Update<'a>
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
    /// use xql::update;
    ///
    /// let query1 = update("book")
    ///     .set("id", 1)
    ///     .returning(["id", "name"]);
    ///
    /// let query2 = update("book")
    ///     .set("id", 1)
    ///     .returning(["id"])
    ///     .returning(["name"]);
    ///
    /// assert_eq!(query1, query2);
    /// ```
    pub fn returning<T>(mut self, returns: T) -> Update<'a>
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
    let someone = &"someone".to_string();
    let query = crate::stmt::update("user")
        .set_values([("id", 1), ("age", 30)])
        .set("name", someone)
        .from(["data"])
        .filter(crate::ops::eq(("user", "id"), ("data", "id")))
        .returning(["id", "age"]);
    assert_eq!(query.to_string(), "UPDATE user SET id = 1, age = 30, name = 'someone' FROM data WHERE user.id = data.id RETURNING id, age");
}
