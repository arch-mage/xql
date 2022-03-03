use crate::clause;
use crate::stmt::result::Result;

/// Binary statement builder.
///
/// Binary statement is two statement combined with `UNION [ALL]`, `EXCEPT
/// [ALL]` or `INTERSECT [ALL]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binary<'a> {
    pub(crate) with: Option<clause::With<'a>>,
    pub(crate) left: Box<Result<'a>>,
    pub(crate) op: &'static str,
    pub(crate) right: Box<Result<'a>>,
}

stmt_common!(Binary);
crate::macros::gen_display!(Binary<'_>);

#[test]
#[cfg(test)]
fn test() {
    use crate::stmt;

    let query = stmt::union(stmt::select([1]), stmt::select([2]));
    assert_eq!(query.to_string(), "SELECT 1 UNION SELECT 2");
    let query = stmt::union_all(stmt::select([1]), stmt::select([2]));
    assert_eq!(query.to_string(), "SELECT 1 UNION ALL SELECT 2");
    let query = stmt::except(stmt::select([1]), stmt::select([2]));
    assert_eq!(query.to_string(), "SELECT 1 EXCEPT SELECT 2");
    let query = stmt::except_all(stmt::select([1]), stmt::select([2]));
    assert_eq!(query.to_string(), "SELECT 1 EXCEPT ALL SELECT 2");
    let query = stmt::intersect(stmt::select([1]), stmt::select([2]));
    assert_eq!(query.to_string(), "SELECT 1 INTERSECT SELECT 2");
    let query = stmt::intersect_all(stmt::select([1]), stmt::select([2]));
    assert_eq!(query.to_string(), "SELECT 1 INTERSECT ALL SELECT 2");
}
