use super::Dialect;
use super::ToSql;

impl<'a> ToSql<'a> for crate::stmt::Stmt<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        match self {
            crate::stmt::Stmt::Insert(stmt) => stmt.build::<D>(sql, args),
            crate::stmt::Stmt::Select(stmt) => stmt.build::<D>(sql, args),
            crate::stmt::Stmt::Update(stmt) => stmt.build::<D>(sql, args),
            crate::stmt::Stmt::Delete(stmt) => stmt.build::<D>(sql, args),
            crate::stmt::Stmt::Values(stmt) => stmt.build::<D>(sql, args),
            crate::stmt::Stmt::Binary(stmt) => stmt.build::<D>(sql, args),
            crate::stmt::Stmt::Result(stmt) => stmt.build::<D>(sql, args),
        }
    }
}

impl<'a> ToSql<'a> for crate::stmt::binary::Binary<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        if let Some(with) = self.with {
            with.build::<D>(sql, args);
            sql.push(' ')
        }
        self.left.build::<D>(sql, args);
        sql.push(' ');
        sql.push_str(self.op);
        sql.push(' ');
        self.right.build::<D>(sql, args);
    }
}

impl<'a> ToSql<'a> for crate::stmt::select::Select<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        if let Some(with) = self.with {
            with.build::<D>(sql, args);
            sql.push(' ')
        }
        macro_rules! build_option {
            ($name:ident) => {
                if let Some($name) = self.$name {
                    sql.push(' ');
                    $name.build::<D>(sql, args);
                };
            };
        }
        self.fields.build::<D>(sql, args);
        build_option!(tables);
        build_option!(filter);
        build_option!(groups);
        build_option!(having);
        build_option!(orders);
    }
}

impl<'a> ToSql<'a> for crate::stmt::insert::Insert<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        if let Some(with) = self.with {
            with.build::<D>(sql, args);
            sql.push(' ')
        }
        self.table.build::<D>(sql, args);
        sql.push(' ');
        self.values.build::<D>(sql, args);
        if let Some(returns) = self.returns {
            sql.push(' ');
            returns.build::<D>(sql, args);
        }
    }
}

impl<'a> ToSql<'a> for crate::stmt::update::Update<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        if let Some(with) = self.with {
            with.build::<D>(sql, args);
            sql.push(' ')
        }
        self.table.build::<D>(sql, args);
        sql.push(' ');
        self.set.build::<D>(sql, args);
        if let Some(from) = self.from {
            sql.push(' ');
            from.build::<D>(sql, args);
        }
        if let Some(filter) = self.filter {
            sql.push(' ');
            filter.build::<D>(sql, args);
        }
        if let Some(returns) = self.returns {
            sql.push(' ');
            returns.build::<D>(sql, args);
        }
    }
}

impl<'a> ToSql<'a> for crate::stmt::delete::Delete<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        if let Some(with) = self.with {
            with.build::<D>(sql, args);
            sql.push(' ')
        }
        self.table.build::<D>(sql, args);
        if let Some(filter) = self.filter {
            sql.push(' ');
            filter.build::<D>(sql, args);
        }
        if let Some(returns) = self.returns {
            sql.push(' ');
            returns.build::<D>(sql, args);
        }
    }
}

impl<'a> ToSql<'a> for crate::stmt::values::Values<'a> {
    #[inline]
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        if let Some(with) = self.with {
            with.build::<D>(sql, args);
            sql.push(' ')
        }
        self.rows.build::<D>(sql, args);
    }
}

impl<'a> ToSql<'a> for crate::stmt::result::Result<'a> {
    #[inline]
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        if let Some(with) = self.with {
            with.build::<D>(sql, args);
            sql.push(' ')
        }
        self.data.build::<D>(sql, args);
        if let Some(limit) = self.limit {
            sql.push(' ');
            limit.build::<D>(sql, args);
        }
        if let Some(offset) = self.offset {
            sql.push(' ');
            offset.build::<D>(sql, args);
        }
    }
}

impl<'a> ToSql<'a> for crate::stmt::data::Data<'a> {
    #[inline]
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>) {
        match self {
            crate::stmt::data::Data::Select(stmt) => stmt.build::<D>(sql, args),
            crate::stmt::data::Data::Values(stmt) => stmt.build::<D>(sql, args),
            crate::stmt::data::Data::Binary(stmt) => stmt.build::<D>(sql, args),
        }
    }
}
