#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use xql_derive::Schema;

pub trait Schema<const N: usize> {
    fn table() -> crate::item::TableRef<'static>;

    fn columns() -> [crate::item::ColumnRef<'static>; N];
}
