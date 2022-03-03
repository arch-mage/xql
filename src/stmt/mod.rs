use crate::clause;
use crate::item::Ident;
use crate::item::Row;
use crate::item::TableRef;

macro_rules! stmt_common {
    ($stmt:ident) => {
        impl<'a> std::convert::From<$stmt<'a>> for $crate::stmt::Stmt<'a> {
            #[inline]
            fn from(val: $stmt<'a>) -> Self {
                $crate::stmt::Stmt::$stmt(val)
            }
        }

        impl<'a> $stmt<'a> {
            pub fn with<N, S>(mut self, name: N, stmt: S) -> $stmt<'a>
            where
                N: Into<$crate::item::Ident<'a>>,
                S: Into<$crate::stmt::Stmt<'a>>,
            {
                self.with = match self.with.take() {
                    Some(mut with) => {
                        with.1.push($crate::item::Cte {
                            name: name.into(),
                            columns: Vec::new(),
                            stmt: stmt.into(),
                        });
                        Some(with)
                    }
                    None => Some(
                        [$crate::item::Cte {
                            name: name.into(),
                            columns: Vec::new(),
                            stmt: stmt.into(),
                        }]
                        .into(),
                    ),
                };
                self
            }

            pub fn with_labeled<N, C, I, S>(mut self, name: N, fields: I, stmt: S) -> $stmt<'a>
            where
                N: Into<$crate::item::Ident<'a>>,
                C: Into<$crate::item::Ident<'a>>,
                I: IntoIterator<Item = C>,
                S: Into<$crate::stmt::Stmt<'a>>,
            {
                self.with = match self.with.take() {
                    Some(mut with) => {
                        with.1.push($crate::item::Cte {
                            name: name.into(),
                            columns: fields.into_iter().map(Into::into).collect(),
                            stmt: stmt.into(),
                        });
                        Some(with)
                    }
                    None => Some(
                        [$crate::item::Cte {
                            name: name.into(),
                            columns: fields.into_iter().map(Into::into).collect(),
                            stmt: stmt.into(),
                        }]
                        .into(),
                    ),
                };
                self
            }

            pub fn recursive(mut self) -> $stmt<'a> {
                if let Some(mut with) = self.with {
                    with.0 = true;
                    self.with = Some(with);
                }
                self
            }

            pub fn no_recursive(mut self) -> $stmt<'a> {
                if let Some(mut with) = self.with {
                    with.0 = false;
                    self.with = Some(with);
                }
                self
            }
        }
    };
}

pub(crate) mod binary;
pub(crate) mod data;
pub(crate) mod delete;
pub(crate) mod insert;
pub(crate) mod result;
pub(crate) mod select;
pub(crate) mod update;
pub(crate) mod values;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt<'a> {
    Insert(insert::Insert<'a>),
    Select(select::Select<'a>),
    Update(update::Update<'a>),
    Delete(delete::Delete<'a>),
    Values(values::Values<'a>),
    Binary(binary::Binary<'a>),
    Result(result::Result<'a>),
}

crate::macros::gen_display!(Stmt<'_>);

/// Construct a `SELECT` statement.
///
/// # Examples
///
/// ```
/// use xql::select;
///
/// assert_eq!(
///     select(("id", "name")).from("book").to_string(),
///     "SELECT id, name FROM book",
/// );
/// ```
#[inline]
pub fn select<'a, F>(fields: F) -> select::Select<'a>
where
    F: Into<clause::Select<'a>>,
{
    select::Select {
        fields: fields.into(),
        ..Default::default()
    }
}

/// Construct a `VALUES` statement.
///
/// # Examples
///
/// ```
/// use xql::values;
///
/// assert_eq!(
///     values([
///         (1, &"Dune".to_string()),
///         (2, &"The Fellowship of the Ring".to_string()),
///     ]).to_string(),
///     "VALUES (1, 'Dune'), (2, 'The Fellowship of the Ring')",
/// );
/// ```
#[inline]
pub fn values<'a, I, R>(values: I) -> values::Values<'a>
where
    R: Into<Row<'a>>,
    I: IntoIterator<Item = R>,
{
    values::Values {
        rows: clause::Values(values.into_iter().map(Into::into).collect()),
        ..Default::default()
    }
}

/// Construct an `INSERT` statement.
///
/// # Examples
///
/// ```
/// use xql::insert;
///
/// assert_eq!(
///     insert("book", ["id", "name"])
///         .values([
///             (1, &"Dune".to_string()),
///             (2, &"The Fellowship of the Ring".to_string()),
///         ])
///         .to_string(),
///     "INSERT INTO book(id, name) VALUES (1, 'Dune'), (2, 'The Fellowship of the Ring')",
/// );
/// ```
#[inline]
pub fn insert<'a, T, I, C>(table: T, columns: I) -> insert::Insert<'a>
where
    T: Into<TableRef<'a>>,
    C: Into<Ident<'a>>,
    I: IntoIterator<Item = C>,
{
    insert::Insert {
        table: clause::Insert(table.into(), columns.into_iter().map(Into::into).collect()),
        ..Default::default()
    }
}

/// Construct a `DELETE` statement.
///
/// # Examples
///
/// ```
/// use xql::delete;
/// use xql::eq;
///
/// assert_eq!(
///     delete("book")
///         .filter(eq("id", 1))
///         .returning(["id", "name"])
///         .to_string(),
///     "DELETE FROM book WHERE id = 1 RETURNING id, name",
/// );
/// ```
#[inline]
pub fn delete<'a, T>(table: T) -> delete::Delete<'a>
where
    T: Into<clause::Delete<'a>>,
{
    delete::Delete {
        table: table.into(),
        ..Default::default()
    }
}

/// Construct an `UPDATE` statement.
///
/// # Examples
///
/// ```
/// use xql::update;
/// use xql::eq;
///
/// assert_eq!(
///     update("book")
///         .set("name", &"The Two Towers".to_string())
///         .filter(eq("id", 2))
///         .returning(["id", "name"])
///         .to_string(),
///     "UPDATE book SET name = 'The Two Towers' WHERE id = 2 RETURNING id, name",
/// );
/// ```
#[inline]
pub fn update<'a, T>(table: T) -> update::Update<'a>
where
    T: Into<clause::Update<'a>>,
{
    update::Update {
        table: table.into(),
        ..Default::default()
    }
}

macro_rules! generate_binary_funcs {
    ($(#[$comment:meta])* $fn:ident $op:expr) => {
        $(#[$comment])*
        #[inline]
        pub fn $fn<'a, L, R>(left: L, right: R) -> $crate::stmt::binary::Binary<'a>
        where
            L: Into<$crate::stmt::result::Result<'a>>,
            R: Into<$crate::stmt::result::Result<'a>>,
        {
            $crate::stmt::binary::Binary {
                with: None,
                op: $op,
                left: ::std::boxed::Box::new(left.into()),
                right: ::std::boxed::Box::new(right.into()),
            }
        }
    };
}

generate_binary_funcs!(
    /// Construct a `UNION` operation on a statement.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::union;
    /// use xql::select;
    /// 
    /// assert_eq!(
    ///     union(select([1]), select([2])).to_string(),
    ///     "SELECT 1 UNION SELECT 2",
    /// );
    /// ```
    union "UNION");
generate_binary_funcs!(
    /// Construct a `UNION ALL` operation on a statement.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::union_all;
    /// use xql::select;
    /// 
    /// assert_eq!(
    ///     union_all(select([1]), select([2])).to_string(),
    ///     "SELECT 1 UNION ALL SELECT 2",
    /// );
    /// ```
    union_all "UNION ALL");
generate_binary_funcs!(
    /// Construct a `EXCEPT` operation on a statement.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::except;
    /// use xql::select;
    /// 
    /// assert_eq!(
    ///     except(select([1]), select([2])).to_string(),
    ///     "SELECT 1 EXCEPT SELECT 2",
    /// );
    /// ```
    except "EXCEPT");
generate_binary_funcs!(
    /// Construct a `EXCEPT ALL` operation on a statement.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::except_all;
    /// use xql::select;
    /// 
    /// assert_eq!(
    ///     except_all(select([1]), select([2])).to_string(),
    ///     "SELECT 1 EXCEPT ALL SELECT 2",
    /// );
    /// ```
    except_all "EXCEPT ALL");
generate_binary_funcs!(
    /// Construct a `INTERSECT` operation on a statement.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::intersect;
    /// use xql::select;
    /// 
    /// assert_eq!(
    ///     intersect(select([1]), select([2])).to_string(),
    ///     "SELECT 1 INTERSECT SELECT 2",
    /// );
    /// ```
    intersect "INTERSECT");
generate_binary_funcs!(
    /// Construct a `INTERSECT ALL` operation on a statement.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use xql::intersect_all;
    /// use xql::select;
    /// 
    /// assert_eq!(
    ///     intersect_all(select([1]), select([2])).to_string(),
    ///     "SELECT 1 INTERSECT ALL SELECT 2",
    /// );
    /// ```
    intersect_all "INTERSECT ALL");

#[cfg(test)]
mod tests {
    #[test]
    fn cte() {
        let tbl1 = &"tbl1".to_string();
        let tbl2 = &"tbl2".to_string();
        let query = crate::stmt::select(["name"])
            .from(["tbl1", "tbl2"])
            .with_labeled("tbl1", ["name"], crate::stmt::values([(tbl1,)]))
            .with(
                "tbl2",
                crate::stmt::select([crate::ops::as_field(tbl2, "name")]),
            );

        assert_eq!(
            query.to_string(),
            "WITH tbl1(name) AS (VALUES ('tbl1')), tbl2 AS (SELECT 'tbl2' AS name) SELECT name FROM tbl1, tbl2"
        );
    }
}
