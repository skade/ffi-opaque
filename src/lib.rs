#[macro_export]
macro_rules! opaque {
    ($name:ident) => {
        #[repr(C)]
        pub struct $name {
            _private: [*mut u8; 0]
        }
    };
}

#[cfg(test)]
pub mod test {
    opaque!(test_t);

    static_assertions::assert_not_impl_all!(test_t: Send, Sync, Unpin);

    #[deny(improper_ctypes, warnings)]
    extern "C" {
        pub fn f(_: *const test_t);
    }
}