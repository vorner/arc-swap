use std::borrow::Borrow;
use std::mem::ManuallyDrop;

use crate::debt::Debt;

mod inner {
    pub trait Sealed {}
}

pub trait Protected<T>: inner::Sealed + Borrow<T> {
}

pub trait Strategy<T>: inner::Sealed {
    type Protected: Protected<T>;
}

struct HybridProtection<T> {
    debt: Option<&'static Debt>,
    ptr: ManuallyDrop<T>,
}
