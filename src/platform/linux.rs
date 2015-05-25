pub const NRBITS: u32 = 8;
pub const TYPEBITS: u32 = 8;

#[cfg(target_arch = "mips")]
mod consts {
    pub const NONE: u8 = 1;
    pub const READ: u8 = 2;
    pub const WRITE: u8 = 4;
    pub const SIZEBITS: u8 = 13;
    pub const DIRBITS: u8 = 3;
}
#[cfg(target_arch = "powerpc")]
mod consts {
    pub const NONE: u8 = 1;
    pub const READ: u8 = 2;
    pub const WRITE: u8 = 4;
    pub const SIZEBITS: u8 = 13;
    pub const DIRBITS: u8 = 3;
}
#[cfg(not(any(target_arch = "powerpc", target_arch = "mips")))]
mod consts {
    pub const NONE: u8 = 0;
    pub const READ: u8 = 2;
    pub const WRITE: u8 = 1;
    pub const SIZEBITS: u8 = 14;
    pub const DIRBITS: u8 = 2;
}

pub use self::consts::*;

pub const NRSHIFT: u32 = 0;
pub const TYPESHIFT: u32 = NRSHIFT + NRBITS as u32;
pub const SIZESHIFT: u32 = TYPESHIFT + SIZEBITS as u32;
pub const DIRSHIFT: u32 = SIZESHIFT + DIRBITS as u32;

pub const NRMASK: u32 = (1 << NRBITS) - 1;
pub const TYPEMASK: u32 = (1 << TYPEBITS) - 1;
pub const SIZEMASK: u32 = (1 << SIZEBITS) - 1;
pub const DIRMASK: u32 = (1 << DIRBITS) - 1;

/// Encode an ioctl command.
#[macro_export]
macro_rules! ioc {
    ($dir:expr, $ty:expr, $nr:expr, $sz:expr) => (
    ($dir as u32) << $crate::DIRBITS |
        ($ty as u32) << $crate::TYPEBITS |
        ($nr as u32) << $crate::NRSHIFT |
        ($sz as u32) << $crate::SIZESHIFT)
}

/// Encode an ioctl command that reads.
#[macro_export]
macro_rules! ior {
    ($ty:expr, $nr:expr, $sz:expr) => (ioc!($crate::READ, $ty, $nr, $sz))
}

/// Encode an ioctl command that writes.
#[macro_export]
macro_rules! iow {
    ($ty:expr, $nr:expr, $sz:expr) => (ioc!($crate::WRITE, $ty, $nr, $sz))
}

/// Encode an ioctl command that both reads and writes.
#[macro_export]
macro_rules! iorw {
    ($ty:expr, $nr:expr, $sz:expr) => (ioc!($crate::READ|$crate::WRITE, $ty, $nr, $sz))
}

/// Declare a wrapper function around an ioctl.
#[macro_export]
macro_rules! ioctl {
    (read $name:ident with $ioty:expr, $nr:expr; $ty:ty) => (
        pub unsafe fn $name(fd: $crate::libc::c_int, val: *mut $ty) -> $crate::libc::c_int {
            $crate::ioctl(fd, ior!($ioty, $nr, ::std::mem::size_of::<$ty>()) as $crate::libc::c_ulong, val)
        }
    );
    (write $name:ident with $ioty:expr, $nr:expr; $ty:ty) => (
        pub unsafe fn $name(fd: $crate::libc::c_int, val: *const $ty) -> $crate::libc::c_int {
            $crate::ioctl(fd, ior!($ioty, $nr, ::std::mem::size_of::<$ty>()) as $crate::libc::c_ulong, val)
        }
    );
    (readwrite $name:ident with $ioty:expr, $nr:expr; $ty:ty) => (
        pub unsafe fn $name(fd: $crate::libc::c_int, val: *mut $ty) -> $crate::libc::c_int {
            $crate::ioctl(fd, iorw!($ioty, $nr, ::std::mem::size_of::<$ty>()) as $crate::libc::c_ulong, val)
        }
    );
    (read buf $name:ident with $ioty:expr, $nr:expr; $ty:ty) => (
        pub unsafe fn $name(fd: $crate::libc::c_int, val: *mut $ty, len: usize) -> $crate::libc::c_int {
            $crate::ioctl(fd, ior!($ioty, $nr, len) as $crate::libc::c_ulong, val)
        }
    );
    (write buf $name:ident with $ioty:expr, $nr:expr; $ty:ty) => (
        pub unsafe fn $name(fd: $crate::libc::c_int, val: *const $ty, len: usize) -> $crate::libc::c_int {
            $crate::ioctl(fd, ior!($ioty, $nr, len) as $crate::libc::c_ulong, val)
        }
    );
    (readwrite buf $name:ident with $ioty:expr, $nr:expr; $ty:ty) => (
        pub unsafe fn $name(fd: $crate::libc::c_int, val: *const $ty, len: usize) -> $crate::libc::c_int {
            $crate::ioctl(fd, iorw!($ioty, $nr, len) as $crate::libc::c_ulong, val)
        }
    );
}

/// Extracts the "direction" (read/write/none) from an encoded ioctl command.
#[inline(always)]
pub fn ioc_dir(nr: u32) -> u8 {
    ((nr >> DIRSHIFT) & DIRMASK) as u8
}

/// Extracts the type from an encoded ioctl command.
#[inline(always)]
pub fn ioc_type(nr: u32) -> u32 {
    (nr >> TYPESHIFT) & TYPEMASK
}

/// Extracts the ioctl number from an encoded ioctl command.
#[inline(always)]
pub fn ioc_nr(nr: u32) -> u32 {
    (nr >> NRSHIFT) & NRMASK
}

/// Extracts the size from an encoded ioctl command.
#[inline(always)]
pub fn ioc_size(nr: u32) -> u32 {
    ((nr >> SIZESHIFT) as u32) & SIZEMASK
}

pub const IN: u32 = (WRITE as u32) << DIRSHIFT;
pub const OUT: u32 = (READ as u32) << DIRSHIFT;
pub const INOUT: u32 = ((READ|WRITE) as u32) << DIRSHIFT;
pub const SIZE_MASK: u32 = SIZEMASK << SIZESHIFT;
