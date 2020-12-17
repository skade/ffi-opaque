#![no_std]

#[doc(hidden)]
pub use core as _core;

/// Creates one or more types capable of representing opaque structs
/// in FFI situations.
///
/// The resulting type:
/// * is zero-sized
/// * cannot be constructed outside of the module it is defined in
/// * has proper alignment
/// * is `!Send`, `!Sync`, `!Unpin`
/// * is FFI safe
///
/// ## Example
///
/// Given the following C headers:
///
/// ```c
/// typedef struct leveldb_options_t leveldb_options_t;
///
/// leveldb_options_t* leveldb_options_create();
/// ```
///
/// We can represent the opaque struct `leveldb_options_t` on the Rust
/// side like this:
///
/// ```rust
/// use ffi_opaque::opaque;
///
/// opaque! {
///     /// And we can document the type.
///     pub struct leveldb_options_t;
/// }
///
/// extern "C" {
///     pub fn leveldb_options_create() -> *mut leveldb_options_t;
/// }
/// ```
///
/// ## Example 2
///
/// Multiple definitions are possible:
///
/// ```rust
/// use ffi_opaque::opaque;
///
/// opaque! {
///     /// Documentation for type_1;
///     pub struct type_1;
///     /// Documentation for type_2;
///     pub struct type_2;
/// }
/// ```
#[macro_export]
macro_rules! opaque {
    ($(
        $(#[$meta:meta])*
        $vis:vis struct $name:ident;
    )+) => {$(
        $(#[$meta])*
        #[repr(C)]
        $vis struct $name {
            // Required for FFI-safe 0-sized type.
            //
            // In the future, this should refer to an extern type.
            // See https://github.com/rust-lang/rust/issues/43467.
            _data: [u8; 0],

            // Required for !Send & !Sync & !Unpin.
            //
            // - `*mut u8` is !Send & !Sync. It must be in `PhantomData` to not
            //   affect alignment.
            //
            // - `PhantomPinned` is !Unpin. It must be in `PhantomData` because
            //   its memory representation is not considered FFI-safe.
            _marker:
                $crate::_core::marker::PhantomData<(*mut u8, $crate::_core::marker::PhantomPinned)>,
        }
    )+};
}

#[cfg(test)]
pub mod test {
    opaque! {
        pub struct test_t;
    }

    static_assertions::assert_not_impl_any!(test_t: Send, Sync, Unpin);

    static_assertions::assert_eq_size!(test_t, [u8; 0]);
    static_assertions::assert_eq_align!(test_t, [u8; 0]);

    #[deny(improper_ctypes, warnings)]
    extern "C" {
        pub fn f(_: *const test_t);
    }
}
