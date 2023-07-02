use std::hash::{Hash, Hasher};

use yarp_sys::yp_constant_t;

use crate::encoding::Encoding;

pub struct Constant<'a> {
    inner: &'a yp_constant_t,
    encoding: Encoding<'a>,
}

impl<'a> Constant<'a> {
    pub(crate) fn new(c_constant: &'a yp_constant_t, encoding: Encoding<'a>) -> Self {
        Self {
            inner: c_constant,
            encoding,
        }
    }
}

impl Hash for Constant<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.inner.hash);
    }
}
