#![allow(non_camel_case_types)]
#![no_std]

pub const STDOUT_FILENO: i32 = 1;

#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

//char is u8 on ARM
pub type c_char = u8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;
pub type size_t = usize;
pub type ssize_t = isize;

extern "C" {
    pub fn write(fd: i32, buf: *const c_void, count: usize) -> isize;
}
