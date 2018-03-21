pub mod imp;

use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
use glib::translate::*;
use glib;
use glib::{IsA,Value};
use glib::object::Downcast;
use glib::signal::{connect,SignalHandlerId};

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

pub trait FooExt {
    fn increment(&self,inc:i32) -> i32;
    fn get_counter(&self) -> i32;
    fn get_name(&self) -> Option<String>;
    fn get_property_name(&self) -> Option<String>;
    fn connect_incremented<F:Fn(&Self,i32,i32)+'static>(&self,f:F) -> SignalHandlerId;
}

impl<O:IsA<Foo> + IsA<glib::object::Object>> FooExt for O {
    fn increment(&self,inc:i32)->i32 {
        unsafe {
            imp::ex_foo_increment(self.to_glib_none().0,inc)
        }
    }
    fn get_counter(&self) -> i32 {
        unimplemented!()
    }

    fn get_name(&self) -> Option<String> {
        unimplemented!()
    }

    fn get_property_name(&self) -> Option<String> {
        unimplemented!()
    }

    fn connect_incremented<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
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

    #[test]
    fn test_counter(){
        let foo = Foo::new(Some("foo's name"));
        let incremented = Rc::new(RefCell::new((0i32,0i32)));
        let incremented_clone = incremented.clone();

    }

}