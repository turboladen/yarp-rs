pub mod program;

use std::ptr::NonNull;

use yarp_sys::yp_node_t;

use crate::{constant::Constant, parser::Parser};

pub struct Ast<'a> {
    inner: NonNull<yp_node_t>,
    parser: &'a Parser,
    constants: Vec<Constant<'a>>,
}

impl<'a> Ast<'a> {
    pub(crate) fn try_new(parser: &'a Parser, node: *mut yp_node_t) -> Result<Self, ()> {
        let inner = match NonNull::new(node) {
            Some(i) => i,
            None => return Err(()),
        };

        let constants: Vec<Constant> = parser
            .constant_pool()
            .iter()
            .filter(|c| c.id != 0)
            .map(|c| Constant::new(c, parser.encoding()))
            .collect();

        Ok(Self {
            inner,
            parser,
            constants,
        })
    }
}

// impl Drop for Ast<'_> {
//     fn drop(&mut self) {
//         unsafe {
//             yp_node_destroy(
//                 self.parser.get_mut().inner_mut(),
//                 self.inner.as_ptr() as *mut yp_node_t,
//             )
//         }
//     }
// }
