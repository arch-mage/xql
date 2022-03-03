use super::Dialect;
use super::ToSql;

impl<'a> ToSql<'a> for crate::value::Value<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        args.push(self);
        D::bind_param(args.len(), self, sql);
    }
}

impl<'a> ToSql<'a> for crate::item::Ident<'a> {
    fn build<D: Dialect>(self, sql: &mut String, _: &mut Vec<crate::value::Value<'a>>) {
        D::quote_ident(self.0, sql);
    }
}

impl<'a> ToSql<'a> for crate::item::Sort {
    #[inline]
    fn build<D: Dialect>(self, sql: &mut String, _: &mut Vec<crate::value::Value<'a>>) {
        match self {
            crate::item::Sort::Asc => sql.push_str("ASC"),
            crate::item::Sort::Desc => sql.push_str("DESC"),
        }
    }
}

impl<'a> ToSql<'a> for crate::item::Order<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        self.0.build::<D>(sql, args);
        if let Some(sort) = self.1 {
            sql.push(' ');
            sort.build::<D>(sql, args);
        }
    }
}

impl<'a> ToSql<'a> for crate::item::Field<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        self.expr.build::<D>(sql, args);
        if let Some(alias) = self.alias {
            sql.push_str(" AS ");
            alias.build::<D>(sql, args);
        }
    }
}

impl<'a> ToSql<'a> for crate::item::Table<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        self.table.build::<D>(sql, args);
        if let Some(alias) = self.alias {
            sql.push_str(" AS ");
            alias.build::<D>(sql, args);
        }
    }
}

impl<'a> ToSql<'a> for crate::item::ColumnRef<'a> {
    #[inline]
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        match self {
            crate::item::ColumnRef::Column(col) => col.build::<D>(sql, args),
            crate::item::ColumnRef::TableColumn(tbl, col) => join!(D, sql, args, ".", [tbl, col]),
            crate::item::ColumnRef::SchemaTableColumn(sch, tbl, col) => {
                join!(D, sql, args, ".", [sch, tbl, col])
            }
        }
    }
}

impl<'a> ToSql<'a> for crate::item::FuncRef<'a> {
    #[inline]
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        match self {
            crate::item::FuncRef::Func(fun) => fun.build::<D>(sql, args),
            crate::item::FuncRef::SchemaFunc(sch, fun) => join!(D, sql, args, ".", [sch, fun]),
        }
    }
}

impl<'a> ToSql<'a> for crate::item::TableRef<'a> {
    #[inline]
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        match self {
            crate::item::TableRef::Table(tbl) => tbl.build::<D>(sql, args),
            crate::item::TableRef::SchemaTable(sch, tbl) => join!(D, sql, args, ".", [sch, tbl]),
        }
    }
}

impl<'a> ToSql<'a> for crate::item::FuncCall<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        self.0.build::<D>(sql, args);
        sql.push('(');
        join!(D, sql, args, ", ", self.1);
        sql.push(')');
    }
}

impl<'a> ToSql<'a> for crate::item::Row<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        sql.push('(');
        join!(D, sql, args, ", ", self.0);
        sql.push(')');
    }
}

impl<'a> ToSql<'a> for crate::item::Cte<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        self.name.build::<D>(sql, args);
        if !self.columns.is_empty() {
            sql.push('(');
            join!(D, sql, args, ", ", self.columns);
            sql.push(')');
        }
        sql.push_str(" AS ");
        sql.push('(');
        self.stmt.build::<D>(sql, args);
        sql.push(')');
    }
}
