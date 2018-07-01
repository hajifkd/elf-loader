#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate cenums;

pub mod file;
pub mod header;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
