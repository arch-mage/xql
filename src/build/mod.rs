macro_rules! join {
    ($Dialect: ty, $sql:expr, $args:expr, $left:expr, $sep:expr, $right:expr, $cond:expr) => {{
        join!($Dialect, $sql, $args, $left, $sep, $right);
        $sql.push_str(" ON ");
        $cond.build::<$Dialect>($sql, $args);
    }};
    ($Dialect: ty, $sql:expr, $args:expr, $left:expr, $sep:expr, $right:expr) => {{
        $left.build::<$Dialect>($sql, $args);
        $sql.push(' ');
        $sql.push_str($sep);
        $sql.push(' ');
        $right.build::<$Dialect>($sql, $args);
    }};

    ($Dialect: ty, $sql:expr, $args:expr, $sep:expr, $elems:expr) => {{
        let mut elems = $elems.into_iter();
        if let Some(elem) = elems.next() {
            elem.build::<$Dialect>($sql, $args);
            for elem in elems {
                $sql.push_str($sep);
                elem.build::<$Dialect>($sql, $args);
            }
        }
    }};
}

mod clause;
mod expr;
mod item;
mod stmt;
mod table_expr;

pub trait Dialect {
    fn quote_literal(val: &str, buff: &mut String);

    fn quote_ident(name: &str, buff: &mut String);

    fn bind_param<'a>(
        n: usize,
        val: crate::value::Value<'a>,
        buff: &mut String,
    ) -> crate::value::Value<'a>;
}

pub(crate) trait ToSql<'a>: Sized {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>);

    fn to_sql<D: Dialect>(self) -> (String, Vec<crate::value::Value<'a>>) {
        let mut sql = String::new();
        let mut args = Vec::new();
        self.build::<D>(&mut sql, &mut args);
        (sql, args)
    }
}

pub(crate) struct Display;

impl Dialect for Display {
    fn quote_literal(val: &str, buff: &mut String) {
        buff.push('\'');
        for ch in val.chars() {
            if ch == '\'' {
                buff.push('\'');
            }
            buff.push(ch);
        }
        buff.push('\'');
    }

    fn quote_ident(name: &str, buff: &mut String) {
        let mut chars = name.chars();
        if chars.next().map(|c| !c.is_ascii_digit()).unwrap_or(true)
            && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            buff.push_str(name);
        } else {
            buff.push('"');
            for ch in name.chars() {
                if ch == '"' {
                    buff.push('"');
                }
                buff.push(ch);
            }
            buff.push('"');
        }
    }

    fn bind_param<'a>(
        _: usize,
        val: crate::value::Value<'a>,
        buff: &mut String,
    ) -> crate::value::Value<'a> {
        use std::fmt::Write;

        match val {
            crate::value::Value::Null(..) => buff.push_str("null"),
            crate::value::Value::Bool(val) => buff.push_str(val.to_string().as_str()),
            crate::value::Value::TinyInt(val) => buff.push_str(val.to_string().as_str()),
            crate::value::Value::SmallInt(val) => buff.push_str(val.to_string().as_str()),
            crate::value::Value::Int(val) => buff.push_str(val.to_string().as_str()),
            crate::value::Value::BigInt(val) => buff.push_str(val.to_string().as_str()),
            crate::value::Value::TinyUInt(val) => buff.push_str(val.to_string().as_str()),
            crate::value::Value::SmallUInt(val) => buff.push_str(val.to_string().as_str()),
            crate::value::Value::UInt(val) => buff.push_str(val.to_string().as_str()),
            crate::value::Value::BigUInt(val) => buff.push_str(val.to_string().as_str()),
            crate::value::Value::Text(val) => Display::quote_literal(val, buff),
            crate::value::Value::Bytes(val) => {
                buff.push_str("b\"");
                for &byte in val.iter() {
                    let _ = write!(buff, "{:02x}", byte);
                }
                buff.push('"');
            }
        };
        val
    }
}
