#[cfg(not(feature="bindings"))]
pub mod imp;

#[cfg(feature="bindings")]
mod ffi;

#[cfg(feature="bindings")]
pub mod imp {
    pub use shared_rstring::ffi::*;
}

use glib_ffi;
use gobject_ffi;
use glib::translate::*;
use std::ptr;
use std::mem;

glib_wrapper!{
    pub struct SharedRString(Shared<imp::SharedRString>);
}