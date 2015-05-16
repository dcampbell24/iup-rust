#![macro_use]

/// Converts a rust string literal into a C null terminated string literal typed `libc::c_char`.
macro_rules! str_to_c_str {
    ($str_lit:expr) => {
        {
            use libc::c_char;
            concat!($str_lit, '\0').as_ptr() as *const c_char
        }
    }
}
