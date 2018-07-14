#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate cenums;

extern crate aligned_alloc;

#[cfg(unix)]
extern crate libc;

#[cfg(windows)]
extern crate winapi;

pub mod file;
pub mod header;
pub mod loader;
mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
