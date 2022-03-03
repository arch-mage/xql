#![deny(missing_debug_implementations, missing_copy_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub mod blanket;
pub mod clause;
pub mod expr;
pub mod func;
pub mod item;
pub mod ops;
pub mod stmt;
pub mod table_expr;
pub mod value;

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
pub mod exec;

mod build;
mod macros;

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
