pub mod imp;

use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
use glib::translate::*;

//pub struct Foo(Option<imp::Foo>);
glib_wrapper!{
    pub struct Foo(Object<imp::Foo>);

    match fn {
        get_type => || imp::ex_foo_get_type(),
    }
}

impl Foo {
    pub fn new(name:Option<&str>) -> Foo {
        unsafe{
            from_glib_full(imp::ex_foo_new(name.to_glib_none().0))
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_new(){
        let foo = Foo::new(Some("foo's name"));
        drop(foo);
    }

}