use crate::rc::{Co, Gen};
use core::{future::Future, pin::Pin};
use alloc::boxed::Box;

/// This is a type alias for generators which can be stored in a `'static`. It's
/// only really needed to help the compiler's type inference along.
#[allow(clippy::module_name_repetitions)]
pub type GenBoxed<Y, R = (), C = ()> =
    Gen<Y, R, Pin<Box<dyn Future<Output = C>>>>;

impl<Y, R, C> GenBoxed<Y, R, C> {
    /// Creates a new generator with a boxed future, so it can be stored in a
    /// `static`.
    ///
    /// This works exactly the same as [`Gen::new`](struct.Gen.html#method.new)
    /// with an immediately boxed future.
    ///
    /// This method exists solely to help the compiler with type inference.
    /// These two lines are equivalent, except that the compiler cannot infer
    /// the correct type on the second line:
    ///
    /// ```compile_fail
    /// # use genawaiter::rc::{Co, Gen, GenBoxed};
    /// # use std::{future::Future, pin::Pin};
    /// #
    /// # async fn producer(co: Co<i32>) {
    /// #     for n in (1..).step_by(2).take_while(|&n| n < 10) { co.yield_(n).await; }
    /// # }
    /// #
    /// let _: GenBoxed<i32> = Gen::new_boxed(|co| producer(co));
    /// let _: GenBoxed<i32> = Gen::new(|co| Box::pin(producer(co)));
    /// ```
    pub fn new_boxed<F>(producer: impl FnOnce(Co<Y, R>) -> F) -> Self
    where
        F: Future<Output = C> + 'static,
    {
        Self::new(|co| Box::pin(producer(co)))
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use crate::{
        ops::GeneratorState,
        rc::{Co, Gen, GenBoxed},
    };
    use core::cell::RefCell;
    use std::thread_local;

    async fn odd_numbers_less_than_ten(mut co: Co<i32>) {
        for n in (1..).step_by(2).take_while(|&n| n < 10) {
            co.yield_(n).await;
        }
    }

    #[test]
    fn can_be_stored_in_static() {
        thread_local! {
            static CEL: RefCell<GenBoxed<i32>> =
                        RefCell::new(Gen::new_boxed(odd_numbers_less_than_ten));
        }

        CEL.with(|cel| {
            let mut guard = cel.borrow_mut();
            assert_eq!(guard.resume(), GeneratorState::Yielded(1));
            assert_eq!(guard.resume(), GeneratorState::Yielded(3));
        });
    }
}
