impl crate::build::Dialect for xql_sqlx_mysql::MySql {
    fn quote_literal(val: &str, buff: &mut String) {
        super::quote(val, '\'', buff)
    }

    fn quote_ident(name: &str, buff: &mut String) {
        super::quote(name, '`', buff)
    }

    fn bind_param(_: usize, buff: &mut String) {
        buff.push('?')
    }
}

impl_sqlx!(xql_sqlx_mysql, MySql);
generate_funcs!(xql_sqlx_mysql, MySql);
