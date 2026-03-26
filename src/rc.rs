/*
   Rc - Reference Counted.
   Rc provides the shared reference to type T , allocated on the heap
   On every clone on Rc , it produces a new reference to the same value on heap
   When last reference is dropped , the inner value is dropped
   You can not obtain a mutable reference to something inside an Rc 
   If you need mutability , use RefCell or Cell

   But Rc never provides a mutable reference

   USE CASE 
   - When one element needs to be present in multiple places
   - Not thread safe

   Rc does not implement Send or Sync
   
   It can not handle cycles . If there are cycles then value will never be dropped

   WEAK AND STRONG POINTERS
   - Weak pointer will not prevent the thing from being deleted
   - Strong pointer will prevent the thing from being deleted

   When strong count becomes 0 .. value is dropped

   You need to upgrade a weak pointer before using it and that upgrade will fail
   if the value is already dropped .. you will get None
*/

use std::ops::Deref;
use crate::cell::Cell;
struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}
pub struct Rc<T> {
    inner: *const RcInner<T>,
    // value: T,
    // ref_count: usize

    // We can not keep ref count here cause each clone will then have its own refcount
    // no way no know when count goes to 0
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });

        Rc { inner: Box::into_raw(inner) }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { &*self.inner };

        let c = inner.refcount.get();
        inner.refcount.set(c + 1);

        Rc {
            inner: self.inner,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        /*
            SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away.
            We have an Rc, therefore the Box has not been deallocated, so deref is fine.
        */
        &(unsafe { &*self.inner }).value
    }
}
