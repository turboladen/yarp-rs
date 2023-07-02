use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct Location {
    inner: Range<usize>,
}

impl Location {
    pub(crate) fn inner_new(start: *const i8, end: *const i8, parser_start: usize) -> Self {
        let start = start.wrapping_sub(parser_start) as usize;
        let end = end.wrapping_sub(parser_start) as usize;

        Self { inner: start..end }
    }

    pub fn as_range(&self) -> &Range<usize> {
        &self.inner
    }
}
