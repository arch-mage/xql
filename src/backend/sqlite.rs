impl crate::build::Dialect for xql_sqlite::Sqlite {
    fn quote_literal(val: &str, buff: &mut String) {
        super::quote(val, '\'', buff)
    }

    fn quote_ident(name: &str, buff: &mut String) {
        super::quote(name, '"', buff)
    }

    fn bind_param(_: usize, buff: &mut String) {
        buff.push('?')
    }
}

impl_sqlx!(xql_sqlite, Sqlite);
generate_funcs!(xql_sqlite, Sqlite);
