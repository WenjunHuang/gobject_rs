#[cfg(not(feature = "bindings"))]
pub mod imp;

use glib_ffi;
use gobject_ffi;
use glib::translate::*;

use std::ptr;
use std::mem;

glib_wrapper! {
pub struct RString(Boxed<imp::RString>);
match fn {
    copy => |ptr| imp::ex_rstring_copy(ptr),
    free => |ptr| imp::ex_rstring_free(ptr),
    get_type => || imp::ex_rstring_get_type(),
}
}