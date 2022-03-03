macro_rules! gen_method {
    ($method:ident) => {
        #[doc = concat!("A short hand for [`xql::stmt::", stringify!($method), "`]", "(crate::stmt::", stringify!($method), ").")]
        #[doc = ""]
        #[doc = "```"]
        #[doc = concat!("use xql::select;")]
        #[doc = concat!("use xql::", stringify!($method), ";")]
        #[doc = concat!("use xql::blanket::ResultExt;")]
        #[doc = ""]
        #[doc = "assert_eq!("]
        #[doc = concat!("    ", "select([1]).", stringify!($method), "(select([2])),")]
        #[doc = concat!("    ", stringify!($method), "(select([1]), select([2])),")]
        #[doc = ");"]
        #[doc = "```"]
        #[inline]
        fn $method<R: Into<$crate::stmt::result::Result<'a>>>(
            self,
            right: R,
        ) -> $crate::stmt::binary::Binary<'a> {
            $crate::stmt::$method(self, right)
        }
    };
}
pub trait ResultExt<'a>: Sized + Into<crate::stmt::result::Result<'a>> {
    gen_method!(union);
    gen_method!(union_all);
    gen_method!(except);
    gen_method!(except_all);
    gen_method!(intersect);
    gen_method!(intersect_all);
}

impl<'a, T> ResultExt<'a> for T where T: Into<crate::stmt::result::Result<'a>> {}

#[test]
#[cfg(test)]
fn test() {
    use crate::stmt::except;
    use crate::stmt::except_all;
    use crate::stmt::intersect;
    use crate::stmt::intersect_all;
    use crate::stmt::union;
    use crate::stmt::union_all;

    use crate::stmt::select;

    assert_eq!(
        select([1]).except(select([2])),
        except(select([1]), select([2]))
    );
    assert_eq!(
        select([1]).except_all(select([2])),
        except_all(select([1]), select([2]))
    );
    assert_eq!(
        select([1]).intersect(select([2])),
        intersect(select([1]), select([2]))
    );
    assert_eq!(
        select([1]).intersect_all(select([2])),
        intersect_all(select([1]), select([2]))
    );
    assert_eq!(
        select([1]).union(select([2])),
        union(select([1]), select([2]))
    );
    assert_eq!(
        select([1]).union_all(select([2])),
        union_all(select([1]), select([2]))
    );
}
