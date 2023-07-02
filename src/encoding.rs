use yarp_sys::yp_encoding_t;

pub struct Encoding<'a> {
    inner: &'a yp_encoding_t,
}

impl<'a> Encoding<'a> {
    pub fn new(inner: &'a yp_encoding_t) -> Self {
        Self { inner }
    }
}
