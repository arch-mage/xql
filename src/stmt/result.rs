use crate::clause;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Result<'a> {
    pub(crate) with: Option<clause::With<'a>>,
    pub(crate) data: crate::stmt::data::Data<'a>,
    pub(crate) limit: Option<clause::Limit>,
    pub(crate) offset: Option<clause::Offset>,
}

stmt_common!(Result);

impl std::fmt::Display for Result<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write_option!(with: self, f, with)?;
        write!(f, "{}", self.data)?;
        write_option!(self, f, limit)?;
        write_option!(self, f, offset)
    }
}

impl<'a, T> std::convert::From<T> for Result<'a>
where
    T: Into<crate::stmt::data::Data<'a>>,
{
    #[inline]
    fn from(val: T) -> Self {
        Result {
            data: val.into(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stmt::select;
    use crate::stmt::values;

    #[test]
    fn limit() {
        let query = select(["id"]).from("data").limit(10);
        assert_eq!(query.to_string(), "SELECT id FROM data LIMIT 10");
        let query = values([(1,), (2,)]).limit(10);
        assert_eq!(query.to_string(), "VALUES (1), (2) LIMIT 10");
    }

    #[test]
    fn offset() {
        let query = select(["id"]).from("data").offset(10);
        assert_eq!(query.to_string(), "SELECT id FROM data OFFSET 10");
        let query = values([(1,), (2,)]).offset(10);
        assert_eq!(query.to_string(), "VALUES (1), (2) OFFSET 10");
    }
}
