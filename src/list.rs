use std::{marker::PhantomData, ptr::NonNull};

use yarp_sys::{yp_comment_t, yp_diagnostic_t, yp_list_node, yp_list_t};

pub(crate) struct ListRef<'a> {
    inner: &'a yp_list_t,
}

impl<'a> ListRef<'a> {
    pub(crate) fn new(inner: &'a yp_list_t) -> Self {
        Self { inner }
    }

    pub(crate) fn head_ptr(&self) -> *const yp_list_node {
        self.inner.head
    }

    pub(crate) fn iter<'b>(&'b self) -> Iter<'a, 'b> {
        Iter::new(self)
    }
}

pub(crate) struct ListNodeRef<'a, 'b: 'a> {
    inner: NonNull<yp_list_node>,
    _marker: PhantomData<&'a ListRef<'b>>,
}

impl<'a, 'b: 'a> ListNodeRef<'a, 'b> {
    pub(crate) fn from_ptr(c_list_node: *const yp_list_node) -> Option<Self> {
        NonNull::new(c_list_node as *mut yp_list_node).map(|inner| Self {
            inner,
            _marker: PhantomData,
        })
    }

    pub(crate) fn new(c_list_node: &mut yp_list_node) -> Self {
        let ptr: *mut yp_list_node = c_list_node;

        Self {
            inner: unsafe { NonNull::new_unchecked(ptr) },
            _marker: PhantomData,
        }
    }

    pub(crate) fn next_ptr(&self) -> *const yp_list_node {
        unsafe { self.inner.as_ref().next }
    }

    pub(crate) fn to_comment_ptr(&self) -> NonNull<yp_comment_t> {
        self.inner.cast::<yp_comment_t>()
    }

    pub(crate) fn to_diagnostic_ptr(&self) -> NonNull<yp_diagnostic_t> {
        self.inner.cast::<yp_diagnostic_t>()
    }
}

pub(crate) struct Iter<'b, 'a: 'b> {
    current: *const yp_list_node,
    _marker: PhantomData<&'b ListRef<'a>>,
}

impl<'b, 'a: 'b> Iter<'a, 'b> {
    pub(crate) fn new(list: &'b ListRef<'a>) -> Self {
        Self {
            current: list.head_ptr(),
            _marker: PhantomData,
        }
    }
}

impl<'a, 'b: 'a> Iterator for Iter<'a, 'b> {
    type Item = ListNodeRef<'a, 'b>;

    fn next(&mut self) -> Option<Self::Item> {
        dbg!(self.current.is_null());
        let current_item = ListNodeRef::from_ptr(self.current)?;
        self.current = current_item.next_ptr();

        Some(current_item)
    }
}
