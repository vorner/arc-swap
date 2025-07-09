use std::mem::{self, ManuallyDrop};

use crate::RefCnt;
use crate::debt::Debt;

pub struct HybridProtection<T: RefCnt> {
    debt: Option<&'static Debt>,
    ptr: ManuallyDrop<T>,
}

impl<T: RefCnt> Drop for HybridProtection<T> {
    #[inline]
    fn drop(&mut self) {
        match self.debt.take() {
            // We have our own copy of Arc, so we don't need a protection. Do nothing (but release
            // the Arc below).
            None => (),
            // If we owed something, just return the debt. We don't have a pointer owned, so
            // nothing to release.
            Some(debt) => {
                let ptr = T::as_ptr(&self.ptr);
                if debt.pay::<T>(ptr) {
                    return;
                }
                // But if the debt was already paid for us, we need to release the pointer, as we
                // were effectively already in the Unprotected mode.
            }
        }
        // Equivalent to T::dec(ptr)
        unsafe { ManuallyDrop::drop(&mut self.ptr) };
    }
}

impl<T, F> InnerStrategy<T> for HybridStrategy<F>
where
    T: RefCnt,
    F: InnerStrategy<T, Protected = T>,
{
pub struct New {

}
