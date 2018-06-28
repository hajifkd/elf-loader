#[macro_use]
extern crate bitflags;

pub mod file;
pub mod header;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
