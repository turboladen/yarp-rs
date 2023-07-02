use std::{marker::PhantomData, ptr::NonNull};

use yarp_sys::{yp_constant_id_t, yp_node_t, yp_node_type_t, yp_program_node_t};

use crate::{location::Location, parser::Parser};

pub struct Program<'a> {
    inner: NonNull<yp_program_node_t>,
    location: Location,
    // parser: Rc<RefCell<&'a mut Parser>>,
    _marker: PhantomData<&'a Parser>,
}

// impl Drop for Program<'_> {
//     fn drop(&mut self) {
//         unsafe {
//             yp_node_destroy(
//                 self.parser.get_mut().inner_mut(),
//                 self.inner.as_ptr() as *mut yp_node_t,
//             )
//         }
//     }
// }

impl<'a> Program<'a> {
    pub(crate) fn try_new(value: *mut yp_node_t, parser_start: usize) -> Result<Self, ()> {
        let inner = match NonNull::new(value) {
            Some(v) => v,
            None => return Err(()),
        };

        if unsafe { inner.as_ref().type_ } == yp_node_type_t::YP_NODE_PROGRAM_NODE {
            let location = unsafe {
                Location::inner_new(
                    inner.as_ref().location.start,
                    inner.as_ref().location.end,
                    parser_start,
                )
            };

            Ok(Self {
                inner: unsafe { NonNull::new_unchecked(inner.as_ptr() as *mut yp_program_node_t) },
                location,
                _marker: PhantomData,
            })
        } else {
            Err(())
        }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn locals(&self) -> &[yp_constant_id_t] {
        unsafe {
            let ids = self.inner.as_ref().locals.ids;

            if ids.is_null() {
                return &[];
            }

            std::slice::from_raw_parts(ids, self.inner.as_ref().locals.size)
        }
    }
}
