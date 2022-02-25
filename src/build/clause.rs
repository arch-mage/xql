use super::Build;
use super::Dialect;

impl<'a> Build<'a> for crate::clause::Select<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("SELECT ");
        join!(D, sql, args, ", ", self.0);
    }
}

impl<'a> Build<'a> for crate::clause::From<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("FROM ");
        join!(D, sql, args, ", ", self.0);
    }
}

impl<'a> Build<'a> for crate::clause::Where<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("WHERE ");
        self.0.build::<D>(sql, args);
    }
}

impl<'a> Build<'a> for crate::clause::GroupBy<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("GROUP BY ");
        join!(D, sql, args, ", ", self.0);
    }
}

impl<'a> Build<'a> for crate::clause::Having<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("HAVING ");
        self.0.build::<D>(sql, args);
    }
}

impl<'a> Build<'a> for crate::clause::OrderBy<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("ORDER BY ");
        join!(D, sql, args, ", ", self.0);
    }
}

impl<'a> Build<'a> for crate::clause::Insert<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("INSERT INTO ");
        self.0.build::<D>(sql, args);
        if !self.1.is_empty() {
            sql.push('(');
            join!(D, sql, args, ", ", self.1);
            sql.push(')');
        }
    }
}

impl<'a> Build<'a> for crate::clause::Values<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("VALUES ");
        join!(D, sql, args, ", ", self.0);
    }
}

impl<'a> Build<'a> for crate::clause::Returning<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("RETURNING ");
        join!(D, sql, args, ", ", self.0);
    }
}

impl<'a> Build<'a> for crate::clause::Update<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("UPDATE ");
        self.0.build::<D>(sql, args);
    }
}

impl<'a> Build<'a> for crate::clause::Delete<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("DELETE FROM ");
        self.0.build::<D>(sql, args);
    }
}

impl<'a> Build<'a> for crate::clause::Set<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("SET ");
        let mut elems = self.0.into_iter();
        if let Some(elem) = elems.next() {
            elem.0.build::<D>(sql, args);
            sql.push_str(" = ");
            elem.1.build::<D>(sql, args);
            for elem in elems {
                sql.push_str(", ");
                elem.0.build::<D>(sql, args);
                sql.push_str(" = ");
                elem.1.build::<D>(sql, args);
            }
        }
    }
}

impl<'a> Build<'a> for crate::clause::With<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        if self.0 {
            sql.push_str("WITH RECURSIVE ");
        } else {
            sql.push_str("WITH ");
        }
        join!(D, sql, args, ", ", self.1);
    }
}

impl<'a> Build<'a> for crate::clause::Limit<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("LIMIT ");
        self.0.build::<D>(sql, args);
    }
}

impl<'a> Build<'a> for crate::clause::Offset<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push_str("OFFSET ");
        self.0.build::<D>(sql, args);
    }
}
