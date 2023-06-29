use std::{borrow::Cow, fmt, ops::Range, ptr::NonNull};

use yarp_sys::{yp_diagnostic_t, yp_parser_t};

use crate::{list::ListNodeRef, to_c_str};

pub struct Diagnostic {
    inner: NonNull<yp_diagnostic_t>,
    location: Range<usize>,
}

impl Diagnostic {
    pub(crate) fn new(inner: NonNull<yp_diagnostic_t>, parser: &yp_parser_t) -> Self {
        let start = unsafe { inner.as_ref().start }.wrapping_sub(parser.start as usize);
        let end = unsafe { inner.as_ref().end }.wrapping_sub(parser.start as usize);

        Self {
            inner,
            location: (start as usize)..(end as usize),
        }
    }

    pub(crate) fn node(&self) -> ListNodeRef<'_, '_> {
        ListNodeRef::new(&mut unsafe { self.inner.as_ref().node })
    }

    pub fn message(&self) -> Cow<'_, str> {
        unsafe { to_c_str(self.inner.as_ref().message) }
    }

    pub fn location(&self) -> &Range<usize> {
        &self.location
    }
}

impl fmt::Debug for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Diagnostic")
            .field("message", &self.message())
            .field("location", &self.location)
            .field("node", &"FIXME")
            .finish()
    }
}

impl PartialEq for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        // TODO: comare nodes
        // self.c_list_node() == other.c_list_node()
        self.location == other.location && self.message() == other.message()
    }
}
