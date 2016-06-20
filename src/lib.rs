#![allow(improper_ctypes, non_camel_case_types)]
#![no_std]


// TODO: STDOUT_FILENO will be changed to c_int at a later time.
// Changing it now would break ctru-rs
pub const STDOUT_FILENO: i32 = 1;

// error stuff
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const EINTR: c_int = 4;
pub const EAGAIN: c_int = 11;
pub const EWOULDBLOCK: c_int = EAGAIN;
pub const EACCES: c_int = 13;
pub const EEXIST: c_int = 17;
pub const EINVAL: c_int = 22;
pub const EPIPE: c_int = 32;
pub const EADDRINUSE: c_int = 98;
pub const EADDRNOTAVAIL: c_int = 99;
pub const ECONNABORTED: c_int = 103;
pub const ECONNRESET: c_int = 104;
pub const ENOTCONN: c_int = 107;
pub const ETIMEDOUT: c_int = 110;
pub const ECONNREFUSED: c_int = 111;

// time stuff
pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;

// FS stuff
pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const F_DUPFD_CLOEXEC: c_int = 1030;
pub const FIOCLEX: c_ulong = 0x5451;
pub const O_NONBLOCK: c_int = 2048;

pub const FD_CLOEXEC: c_int = 0x1;

pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_RDWR: c_int = 2;
pub const O_ACCMODE: c_int = 3;
pub const O_CREAT: c_int = 64;
pub const O_EXCL: c_int = 128;
pub const O_TRUNC: c_int = 512;
pub const O_APPEND: c_int = 1024;
pub const O_CLOEXEC: c_int = 0x80000;

pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

pub const DT_FIFO: u8 = 1;
pub const DT_CHR: u8 = 2;
pub const DT_DIR: u8 = 4;
pub const DT_BLK: u8 = 6;
pub const DT_REG: u8 = 8;
pub const DT_LNK: u8 = 10;
pub const DT_SOCK: u8 = 12;

pub const S_IFIFO: mode_t = 4096;
pub const S_IFCHR: mode_t = 8192;
pub const S_IFDIR: mode_t = 16384;
pub const S_IFBLK: mode_t = 24576;
pub const S_IFREG: mode_t = 32768;
pub const S_IFLNK: mode_t = 40960;
pub const S_IFSOCK: mode_t = 49152;
pub const S_IFMT: mode_t = 61440;

#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

// char is u8 on ARM
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

pub type time_t = i32;
pub type clockid_t = c_int;
pub type mode_t = u32;
pub type sighandler_t = size_t;

pub type dev_t = u64;
pub type nlink_t = u32;
pub type uid_t = u32;
pub type gid_t = u32;

pub type off_t = i32;
pub type blksize_t = i32;
pub type blkcnt64_t = i64;
pub type ino_t = u32;
pub type suseconds_t = i32;

pub enum DIR {}

#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt64_t,
    pub st_atime: time_t,
    pub st_atime_nsec: c_long,
    pub st_mtime: time_t,
    pub st_mtime_nsec: c_long,
    pub st_ctime: time_t,
    pub st_ctime_nsec: c_long,
    pub st_ino: ino_t,
}

pub struct dirent {
    pub d_ino: ino_t,
    pub d_off: off_t,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256],
}

extern "C" {
    pub fn chmod(path: *const c_char, mode: mode_t) -> c_int;
    pub fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn closedir(dirp: *mut DIR) -> c_int;
    pub fn __errno() -> *const c_int;
    pub fn exit(status: c_int) -> !;
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    pub fn fdatasync(fd: c_int) -> c_int;
    pub fn free(p: *mut c_void);
    pub fn fstat(fildes: c_int, buf: *mut stat) -> c_int;
    pub fn ftruncate(fd: c_int, length: off_t) -> c_int;
    pub fn fsync(fd: c_int) -> c_int;
    pub fn gettimeofday(tp: *mut timeval, tz: *mut c_void) -> c_int;
    pub fn link(src: *const c_char, dst: *const c_char) -> c_int;
    pub fn lstat(path: *const c_char, buf: *mut stat) -> c_int;
    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn mkdir(path: *const c_char, mode: mode_t) -> c_int;
    pub fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
    pub fn opendir(dirname: *const c_char) -> *mut DIR;
    pub fn read(fd: c_int, puf: *mut c_void, count: size_t) -> ssize_t;
    pub fn readlink(path: *const c_char, buf: *mut c_char, bufsz: size_t) -> ssize_t;
    pub fn readdir_r(dirp: *mut DIR, entry: *mut dirent, result: *mut *mut dirent) -> c_int;
    pub fn realpath(pathname: *const c_char, resolved: *mut c_char) -> *mut c_char;
    pub fn rename(oldname: *const c_char, newname: *const c_char) -> c_int;
    pub fn rmdir(path: *const c_char) -> c_int;
    pub fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    pub fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    pub fn strlen(cs: *const c_char) -> size_t;
    pub fn symlink(path1: *const c_char, path2: *const c_char) -> c_int;
    pub fn unlink(c: *const c_char) -> c_int;
    // TODO: write() will be converted to use c types at a later time.
    // Doing so right now would break ctru-rs
    pub fn write(fd: i32, buf: *const c_void, count: usize) -> isize;
}
