use glib_sys as glib_ffi;
use gobject_sys as gobject_ffi;

use glib::translate::*;
use std::sync::*;
use std::ffi::*;
use std::os::raw::*;
use std::mem;

#[derive(Clone)]
pub struct RString(Option<String>);

impl RString {
    fn new(s: Option<String>) -> RString {
        RString(s)
    }

    fn get(&self) -> Option<String> {
        self.0.clone()
    }

    fn set(&mut self, s: Option<String>) {
        self.0 = s;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_get_type() -> glib_ffi::GType {
    static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
    static ONCE: Once = ONCE_INIT;

    ONCE.call_once(|| {
        let type_name = CString::new("ExRString").unwrap();

        TYPE = gobject_ffi::g_boxed_type_register_static(
            type_name.as_ptr(),
            Some(mem::transmute(ex_rstring_copy as *const c_void)),
            Some(mem::transmute(ex_rstring_free as *const c_void)),
        );
    });
    TYPE
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_name(s: *const c_char) -> *mut RString {
    let s = Box::new(RString::new(from_glib_none(s)));
    Box::into_raw(s)
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_copy(rstring: *const RString) -> *mut RString {
    let rstring = &*rstring;
    let s = Box::new(rstring.clone());
    Box::into_raw(s)
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_free(rstring: *mut RString) {
    let _ = Box::from_raw(rstring);
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_get(rstring: *const RString) -> *mut c_char {
    let rstring = &*rstring;
    rstring.get().to_glib_full()
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_set(rstring: *mut RString, s: *const c_char) {
    let rstring = &mut *rstring;
    rstring.set(from_glib_none(s));
}

pub struct SharedRString(Option<String>);

impl SharedRString {
    fn new(s: Option<String>) -> Self {
        SharedRString(s)
    }
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_new(s: *const c_char) -> *mut SharedRString {
    let s = Arc::new(SharedRString::new(from_glib_none(s)));
    Arc::into_raw(s) as *mut _
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_ref(shared_rstring:*mut SharedRString)->*mut SharedRString {
    let shared_rstring = Arc::from_raw(shared_rstring);
    let s = shared_rstring.clone();

    let _ = Arc::into_raw(shared_rstring);
    Arc::into_raw(s) as *mut _
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_unref(shared_rstring: *mut SharedRString) {
    let _ = Arc::from_raw(shared_rstring);
}