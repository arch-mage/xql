use std::fmt::Write;

impl crate::build::Dialect for xql_postgres::Postgres {
    fn quote_literal(val: &str, buff: &mut String) {
        super::quote(val, '\'', buff)
    }

    fn quote_ident(name: &str, buff: &mut String) {
        super::quote(name, '"', buff)
    }

    fn bind_param(n: usize, buff: &mut String) {
        let _ = write!(buff, "${n}");
    }
}

impl_sqlx!(xql_postgres, Postgres);
generate_funcs!(xql_postgres, Postgres);
