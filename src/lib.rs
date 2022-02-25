#![deny(missing_debug_implementations, missing_copy_implementations)]
#![doc = include_str!("../README.md")]

mod backend;
mod clause;
mod expr;
mod func;
mod item;
mod ops;
mod stmt;
mod table_expr;
mod value;

mod build;
mod macros;
mod utils;

// re exports statement builder
pub use stmt::delete;
pub use stmt::insert;
pub use stmt::select;
pub use stmt::update;
pub use stmt::values;
pub use stmt::{except, except_all, intersect, intersect_all, union, union_all};

// re exports functions
pub use func::func;
pub use func::{avg, count, max, min, sum};

// re exports ops
pub use ops::{add, div, mul, rem, sub};
pub use ops::{and, not, or};
pub use ops::{as_field, as_table, asc, desc, paren};
pub use ops::{binop, postop, preop};
pub use ops::{eq, ge, gt, le, lt, ne};
pub use ops::{ilike, isnull, like};

pub use ops::{cross_join, join, natural_join};
pub use ops::{full_join, left_join, right_join};
pub use ops::{natural_full_join, natural_left_join, natural_right_join};

// stmt
pub use stmt::delete::Delete;
pub use stmt::insert::Insert;
pub use stmt::select::Select;
pub use stmt::update::Update;
pub use stmt::values::Values;

pub use backend::Query;

#[cfg(feature = "postgres")]
pub mod postgres {
    pub use crate::backend::postgres::fetch_one;
    pub use crate::backend::postgres::fetch_one_as;
    pub use crate::backend::postgres::fetch_one_scalar;

    pub use crate::backend::postgres::fetch_all;
    pub use crate::backend::postgres::fetch_all_as;
    pub use crate::backend::postgres::fetch_all_scalar;

    pub use crate::backend::postgres::fetch_optional;
    pub use crate::backend::postgres::fetch_optional_as;
    pub use crate::backend::postgres::fetch_optional_scalar;
}

#[cfg(feature = "mysql")]
pub mod mysql {
    pub use crate::backend::mysql::fetch_one;
    pub use crate::backend::mysql::fetch_one_as;
    pub use crate::backend::mysql::fetch_one_scalar;

    pub use crate::backend::mysql::fetch_all;
    pub use crate::backend::mysql::fetch_all_as;
    pub use crate::backend::mysql::fetch_all_scalar;

    pub use crate::backend::mysql::fetch_optional;
    pub use crate::backend::mysql::fetch_optional_as;
    pub use crate::backend::mysql::fetch_optional_scalar;
}

#[cfg(feature = "sqlite")]
pub mod sqlite {
    pub use crate::backend::sqlite::fetch_one;
    pub use crate::backend::sqlite::fetch_one_as;
    pub use crate::backend::sqlite::fetch_one_scalar;

    pub use crate::backend::sqlite::fetch_all;
    pub use crate::backend::sqlite::fetch_all_as;
    pub use crate::backend::sqlite::fetch_all_scalar;

    pub use crate::backend::sqlite::fetch_optional;
    pub use crate::backend::sqlite::fetch_optional_as;
    pub use crate::backend::sqlite::fetch_optional_scalar;
}
