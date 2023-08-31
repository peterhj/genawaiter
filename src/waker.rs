use core::{
    mem::ManuallyDrop,
    ops::Deref,
    ptr,
    task::{RawWaker, RawWakerVTable, Waker},
};

#[inline]
pub fn create() -> impl Deref<Target = Waker> {
    // SAFETY: The waker points to a vtable with functions
    // that do nothing, including a no-op drop, as well as
    // a null data pointer. So, there is nothing to do,
    // and there is nothing to drop.
    ManuallyDrop::new(unsafe { Waker::from_raw(RAW_WAKER) })
}

const RAW_WAKER: RawWaker = RawWaker::new(ptr::null(), &VTABLE);
const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

unsafe fn clone(_: *const ()) -> RawWaker {
    RAW_WAKER
}

unsafe fn wake(_: *const ()) {}

unsafe fn wake_by_ref(_: *const ()) {}

unsafe fn drop(_: *const ()) {}
