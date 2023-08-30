/**
Creates a generator on the stack.

This macro is deprecated. Use [`let_gen!`] or [`let_gen_using!`] instead.

[`let_gen!`]: stack/macro.let_gen.html
[`let_gen_using!`]: stack/macro.let_gen_using.html
*/
#[macro_export]
#[deprecated = "Use `let_gen_using!()` instead."]
macro_rules! generator_mut {
    ($name:ident, $producer:expr $(,)?) => {
        $crate::stack::let_gen_using!($name, $producer);
    };
}

/**
Creates a generator on the stack unsafely.

This macro is deprecated. Use [`let_gen!`] or [`let_gen_using!`] instead.

[`let_gen!`]: stack/macro.let_gen.html
[`let_gen_using!`]: stack/macro.let_gen_using.html
*/
#[macro_export]
#[deprecated = "Use `let_gen_using!()` instead."]
macro_rules! unsafe_create_generator {
    ($name:ident, $producer:expr $(,)?) => {
        let mut generator_state = $crate::stack::Shelf::new();
        #[allow(unused_mut)]
        let mut $name =
            unsafe { $crate::stack::Gen::new(&mut generator_state, $producer) };
    };
}
