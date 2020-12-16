#![no_std]

#[doc(hidden)]
pub use core as _core;

#[macro_export]
macro_rules! opaque {
    ($name:ident) => {
        #[repr(C)]
        pub struct $name {
            // Required for FFI-safe 0-sized type.
            //
            // In the future, this should refer to an extern type.
            // See https://github.com/rust-lang/rust/issues/43467.
            _private: [u8; 0],

            // Required for !Send & !Sync & !Unpin.
            //
            // - `*mut u8` is !Send & !Sync. It must be in `PhantomData` to not
            //   affect alignment.
            //
            // - `PhantomPinned` is !Unpin. It must be in `PhantomData` because
            //   its memory representation is not considered FFI-safe.
            _pointer_marker:
                $crate::_core::marker::PhantomData<(*mut u8, $crate::_core::marker::PhantomPinned)>,
        }
    };
}

#[cfg(test)]
pub mod test {
    opaque!(test_t);

    static_assertions::assert_not_impl_any!(test_t: Send, Sync, Unpin);

    #[deny(improper_ctypes, warnings)]
    extern "C" {
        pub fn f(_: *const test_t);
    }
}
