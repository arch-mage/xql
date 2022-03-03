use crate::clause;
use crate::stmt::result::Result;

/// `VALUES` statement builder.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Values<'a> {
    pub(crate) with: Option<clause::With<'a>>,
    pub(crate) rows: clause::Values<'a>,
}

stmt_common!(Values);

crate::macros::gen_display!(Values<'_>);

impl<'a> Values<'a> {
    pub fn limit(self, limit: u32) -> Result<'a> {
        Result {
            data: self.into(),
            limit: Some(clause::Limit(limit)),
            ..Default::default()
        }
    }

    pub fn offset(self, offset: u32) -> Result<'a> {
        Result {
            data: self.into(),
            offset: Some(clause::Offset(offset)),
            ..Default::default()
        }
    }
}
