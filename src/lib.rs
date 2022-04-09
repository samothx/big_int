#![crate_name = "simple_big_int"]

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

#[cfg(feature = "big_uint")]
pub mod big_uint;
#[cfg(feature = "big_uint")]
pub use big_uint::BigUInt;

#[cfg(feature = "big_int")]
pub mod big_int;
#[cfg(feature = "big_int")]
pub use big_int::BigInt;

#[cfg(feature = "rational")]
pub mod rational;
#[cfg(feature = "rational")]
pub use rational::Rational;







