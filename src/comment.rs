use std::{
    fmt::{self, Debug},
    ops::Range,
    ptr::NonNull,
};

use yarp_sys::{yp_comment_t, yp_comment_type_t, yp_parser_t};

pub struct Comment {
    inner: NonNull<yp_comment_t>,
    location: Range<usize>,
}

impl Comment {
    pub(crate) fn inner_new(inner: NonNull<yp_comment_t>, parser: &yp_parser_t) -> Self {
        let start = unsafe { inner.as_ref().start }.wrapping_sub(parser.start as usize);
        let end = unsafe { inner.as_ref().end }.wrapping_sub(parser.start as usize);

        Self {
            inner,
            location: (start as usize)..(end as usize),
        }
    }

    pub fn location(&self) -> &Range<usize> {
        &self.location
    }

    pub fn type_(&self) -> CommentType {
        match unsafe { self.inner.as_ref().type_ } {
            yp_comment_type_t::YP_COMMENT_INLINE => CommentType::Inline,
            yp_comment_type_t::YP_COMMENT_EMBDOC => CommentType::Embdoc,
            yp_comment_type_t::YP_COMMENT___END__ => CommentType::End,
            t => panic!("Unknown comment type: {t:?}"),
        }
    }

    fn c_type(&self) -> yp_comment_type_t {
        unsafe { self.inner.as_ref().type_ }
    }
}

impl Debug for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Comment")
            .field("type", &self.type_())
            .field("location", &self.location)
            .finish()
    }
}

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.c_type() == other.c_type() && self.location == other.location
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommentType {
    Inline,
    Embdoc,
    End,
}
