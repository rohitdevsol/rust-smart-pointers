/*
  A mutable memory location with dynamically checked borrow rules
  Can be used when you know that there will be know cycles during recursion
  RefCell is a good use case for graph and trees

  It is also not !sync
 */

use std::{ cell::UnsafeCell, ops::{ Deref, DerefMut } };

use crate::cell::Cell;

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // SAFETY: No exclusive references have been given out since state would be exclusive
                // Some(unsafe { &*self.value.get() })
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                // SAFETY: No exclusive references have been given out since state would be exclusive
                // Some(unsafe { &*self.value.get() })
                Some(Ref { refcell: self })
            }
            _ => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            //SAFETY: No other references have been given out since state would be shared or exclusive
            self.state.set(RefState::Exclusive);
            // Some(unsafe { &mut *self.value.get() })
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

// But the api end needs value not this whole struct

impl<T> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        /*
            SAFETY: 
            1. A ref is only created if no exclusive references are given out
            2. Once a ref is given out, state becomes shared, no exclusive references are given out
            So, dereferencing is into a shared reference is fine.
         */
        unsafe {
            &*self.refcell.value.get()
        }
    }
}
impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(n) => {
                if n == 1 {
                    self.refcell.state.set(RefState::Unshared);
                } else {
                    self.refcell.state.set(RefState::Shared(n - 1));
                }
            }
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        /*
            SAFETY: Check out DerefMut
         */
        unsafe {
            &*self.refcell.value.get()
        }
    }
}
// but it needs to implement the DerefMut too
// it would be bizarre and inconsistent if a type supported *x = val but not let _ = *x

impl<T> DerefMut for RefMut<'_, T> {
    // type Target = T;
    fn deref_mut(&mut self) -> &mut Self::Target {
        /*
            SAFETY: 
            1. A RefMut is only created when no other references are given out
            2. Once RefMut is given out, state becomes exclusive, no future references are given out
            So, we have an exclusive lease on the inner value, so mutably dereferencing is safe
         */
        unsafe {
            &mut *self.refcell.value.get()
        }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Unshared | RefState::Shared(_) => unreachable!(),
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
        }
    }
}
