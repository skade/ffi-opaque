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
    opaque!(leveldb_t);

    static_assertions::assert_not_impl_all!(leveldb_t: Send, Sync, Unpin);

    #[deny(improper_ctypes, warnings)]
    extern "C" {
        pub fn f(_: *const leveldb_t);
    }
}