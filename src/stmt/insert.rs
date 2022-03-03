use crate::clause;
use crate::item::Row;

/// `INSERT` statement builder.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Insert<'a> {
    pub(crate) with: Option<clause::With<'a>>,
    pub(crate) table: clause::Insert<'a>,
    pub(crate) values: crate::stmt::data::Data<'a>,
    pub(crate) returns: Option<clause::Returning<'a>>,
}

stmt_common!(Insert);

impl<'a> Insert<'a> {
    pub fn values<I, R>(mut self, values: I) -> Insert<'a>
    where
        R: Into<Row<'a>>,
        I: IntoIterator<Item = R>,
    {
        self.values = match self.values {
            crate::stmt::data::Data::Values(mut inner) => {
                inner.rows.0.extend(values.into_iter().map(Into::into));
                crate::stmt::data::Data::Values(inner)
            }
            _ => unreachable!(),
        };
        self
    }

    pub fn select<T>(mut self, select: T) -> Insert<'a>
    where
        T: Into<crate::stmt::select::Select<'a>>,
    {
        self.values = crate::stmt::data::Data::Select(Box::new(select.into()));
        self
    }

    pub fn returning<T>(mut self, returns: T) -> Insert<'a>
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

crate::macros::gen_display!(Insert<'_>);

#[test]
#[cfg(test)]
fn test() {
    let row1 = (1, &"John".to_string());
    let row2 = (2, &"Jack".to_string());
    let query = crate::stmt::insert("user", ["id", "name"])
        .values([row1, row2])
        .returning(["id", "name"]);
    assert_eq!(
        query.to_string(),
        "INSERT INTO user(id, name) VALUES (1, \'John\'), (2, \'Jack\') RETURNING id, name"
    );

    let row3 = (1, &"name".to_string());
    let query = crate::stmt::insert("user", ["id", "name"])
        .select(crate::stmt::select(row3))
        .returning(["id", "name"]);

    assert_eq!(
        query.to_string(),
        "INSERT INTO user(id, name) SELECT 1, 'name' RETURNING id, name"
    );
}
