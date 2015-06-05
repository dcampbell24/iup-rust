//! Guard for automatically destroying dettached elements on drop.
use std::mem::forget;
use Element;
use std::ops::{Deref, DerefMut};

/// Guards an element by effectively destroying it on drop.
///
/// This **does not** provide ownership semantics *(like cell types or smart pointers)*, just a
/// simple dropper for the wrapped element.
///
/// The programmer must make there's no copies of the element flying around the program.
/// 
/// It's recommended to use this only for resource elements (those elements that cannot be attached
/// to a dialog in any meaningful way).
///
/// It's undefined behaviour to manually destroy a element that is wrapped within a guard, unless
/// the elements gets `unwrap`ped before `Drop` gets called. Consequently it is also undefined
/// behaviour to have two guards guarding the same element.
///
/// Please refer to the crate level documentation of IUP-Rust (the main doc page) for details on
/// ownership of elements.
#[derive(Debug)]
pub struct Guard<E: Element>(E);

impl<E: Element> Guard<E> {
    /// Creates a guard for the specified element.
    pub fn new(element: E) -> Guard<E> {
        Guard(element)
    }

    /// Forgets this guard and unwraps out the contained element.
    pub fn unwrap(self) -> E {
        let inner = self.0;
        forget(self);   // Don't drop me or I'll destroy `inner`!
        inner
    }
}

impl<E: Element> Drop for Guard<E> {
    fn drop(&mut self) {
        self.0.destroy()
    }
}

/// Be careful on deferecing so you don't store another copy of the element somewhere.
impl<E: Element> Deref for Guard<E> {
    type Target = E;
    fn deref(&self) -> &E {
        &self.0
    }
}

/// Be careful on deferecing so you don't store another copy of the element somewhere.
impl<E: Element> DerefMut for Guard<E> {
    fn deref_mut(&mut self) -> &mut E {
        &mut self.0
    }
}
