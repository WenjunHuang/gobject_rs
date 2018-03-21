use glib_ffi;
use gobject_ffi;

use std::ffi::CString;
use std::sync::{Once, ONCE_INIT};
use std::mem;
use std::sync::Arc;

use glib::translate::{from_glib_none, ToGlibPtr};
use std::os::raw::{c_char, c_void};

pub struct SharedRString(Option<String>);

impl SharedRString {
    fn new(s: Option<String>) -> Arc<SharedRString> {
        Arc::new(SharedRString(s))
    }

    fn get(&self) -> Option<String> {
        self.0.clone();
    }
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_new(s: *const c_char) -> *mut SharedRString {
    let s = SharedRString::new(from_glib_none(s));
    Arc::into_raw(s) as *mut _
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_ref(shared_rstring: *mut SharedRString,) -> *mut SharedRString {
    let shared_rstring = Arc::from_raw(shared_rstring);
    let s = shared_rstring.clone();

    let _ = Arc::into_raw(shared_rstring);

    Arc::into_raw(s) as *mut _
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_unref(shared_rstring: *mut SharedRString){
    let _ = Arc::from_raw(shared_rstring);
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_get(shared_rstring:*mut SharedRString)->*mut c_char {
    let shared_rstring = &*shared_rstring;
    shared_rstring.get().to_glib_full()
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_get_type() -> glib_ffi::GType {
    static TYPE:glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
    static ONCE:Once = ONCE_INIT;

    ONCE.call_once(||{
        let type_name = CString::new("ExSharedRString").unwrap();

        TYPE = gobject_ffi::g_boxed_type_register_static(
            type_name.as_ptr(),
            Some(mem::transmute(ex_shared_rstring_ref as *const c_void)),
            Some(mem::transmute(ex_shared_rstring_unref as *const c_void)),
        );
    });

    TYPE
}