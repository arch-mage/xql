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

pub trait Dialect {
    fn quote_literal(val: &str, buff: &mut String);

    fn quote_ident(name: &str, buff: &mut String);

    fn bind_param(n: usize, buff: &mut String);
}

pub(crate) trait Build<'a> {
    fn build<D: Dialect>(self, sql: &mut String, args: &mut Vec<crate::value::Value<'a>>);
}

mod clause;
mod expr;
mod item;
mod stmt;
mod table_expr;
