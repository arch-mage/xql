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

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
pub trait StmtExt<'v>: Sized + Into<crate::stmt::Stmt<'v>> {
    fn fetch_one<'a, 'c, DB, E>(
        self,
        executor: E,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = Result<DB::Row, sqlx::Error>>>>
    where
        Self: 'a,
        'v: 'a,
        DB: sqlx::Database + crate::exec::Backend + crate::build::Dialect,
        E: 'a + sqlx::Executor<'c, Database = DB>,
    {
        Box::pin(async move { crate::exec::fetch_one(self, executor).await })
    }

    #[allow(clippy::type_complexity)]
    fn fetch_optional<'a, 'c, DB, E>(
        self,
        executor: E,
    ) -> std::pin::Pin<
        Box<dyn 'a + std::future::Future<Output = Result<Option<DB::Row>, sqlx::Error>>>,
    >
    where
        Self: 'a,
        'v: 'a,
        DB: sqlx::Database + crate::exec::Backend + crate::build::Dialect,
        E: 'a + sqlx::Executor<'c, Database = DB>,
    {
        Box::pin(async move { crate::exec::fetch_optional(self, executor).await })
    }

    #[allow(clippy::type_complexity)]
    fn fetch_all<'a, 'c, DB, E>(
        self,
        executor: E,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = Result<Vec<DB::Row>, sqlx::Error>>>>
    where
        Self: 'a,
        'v: 'a,
        DB: sqlx::Database + crate::exec::Backend + crate::build::Dialect,
        E: 'a + sqlx::Executor<'c, Database = DB>,
    {
        Box::pin(async move { crate::exec::fetch_all(self, executor).await })
    }

    fn fetch_one_as<'a, 'c, E, O>(
        self,
        executor: E,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = Result<O, sqlx::Error>>>>
    where
        Self: 'a,
        'v: 'a,
        E::Database: sqlx::Database + crate::exec::Backend + crate::build::Dialect,
        E: 'a + sqlx::Executor<'c>,
        O: Send + Unpin + for<'r> sqlx::FromRow<'r, <E::Database as sqlx::Database>::Row>,
    {
        Box::pin(async move { crate::exec::fetch_one_as(self, executor).await })
    }

    fn fetch_optional_as<'a, 'c, O, DB, E>(
        self,
        executor: E,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = Result<Option<O>, sqlx::Error>>>>
    where
        Self: 'a,
        'v: 'a,
        DB: sqlx::Database + crate::exec::Backend + crate::build::Dialect,
        E: 'a + sqlx::Executor<'c, Database = DB>,
        O: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        Box::pin(async move { crate::exec::fetch_optional_as(self, executor).await })
    }

    fn fetch_all_as<'a, 'c, O, DB, E>(
        self,
        executor: E,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = Result<Vec<O>, sqlx::Error>>>>
    where
        Self: 'a,
        'v: 'a,
        DB: sqlx::Database + crate::exec::Backend + crate::build::Dialect,
        E: 'a + sqlx::Executor<'c, Database = DB>,
        O: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        Box::pin(async move { crate::exec::fetch_all_as(self, executor).await })
    }

    fn fetch_one_scalar<'a, 'c, E, O>(
        self,
        executor: E,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = Result<O, sqlx::Error>>>>
    where
        Self: 'a,
        'v: 'a,
        E::Database: sqlx::Database + crate::exec::Backend + crate::build::Dialect,
        E: 'a + sqlx::Executor<'c>,
        O: Send + Unpin,
        (O,): for<'r> sqlx::FromRow<'r, <E::Database as sqlx::Database>::Row>,
    {
        Box::pin(async move { crate::exec::fetch_one_scalar(self, executor).await })
    }

    fn fetch_optional_scalar<'a, 'c, O, DB, E>(
        self,
        executor: E,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = Result<Option<O>, sqlx::Error>>>>
    where
        Self: 'a,
        'v: 'a,
        DB: sqlx::Database + crate::exec::Backend + crate::build::Dialect,
        E: 'a + sqlx::Executor<'c, Database = DB>,
        O: Send + Unpin,
        (O,): for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        Box::pin(async move { crate::exec::fetch_optional_scalar(self, executor).await })
    }

    fn fetch_all_scalar<'a, 'c, O, DB, E>(
        self,
        executor: E,
    ) -> std::pin::Pin<Box<dyn 'a + std::future::Future<Output = Result<Vec<O>, sqlx::Error>>>>
    where
        Self: 'a,
        'v: 'a,
        DB: sqlx::Database + crate::exec::Backend + crate::build::Dialect,
        E: 'a + sqlx::Executor<'c, Database = DB>,
        O: Send + Unpin,
        (O,): for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        Box::pin(async move { crate::exec::fetch_all_scalar(self, executor).await })
    }
}

#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
impl<'a, T> StmtExt<'a> for T where T: Into<crate::stmt::Stmt<'a>> {}

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
