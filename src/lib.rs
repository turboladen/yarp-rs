pub mod ast;
pub mod comment;
pub mod constant;
pub mod diagnostic;
pub mod encoding;
pub mod error;
pub(crate) mod list;
pub mod location;
pub mod parser;

pub(crate) fn to_c_str(attr: *const std::ffi::c_char) -> std::borrow::Cow<'static, str> {
    let c_str = unsafe { std::ffi::CStr::from_ptr(attr) };

    c_str.to_string_lossy()
}
