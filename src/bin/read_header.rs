extern crate elf_loader;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::mem;
use std::slice;

use elf_loader::header::ElfHeader;

fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let bin_file = File::open(&args[1])?;
    let mut reader = BufReader::new(bin_file);

    let mut header: ElfHeader = unsafe { mem::zeroed() };
    let header_size = mem::size_of::<ElfHeader>();
    unsafe {
        let header_slice = slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, header_size);
        reader.read_exact(header_slice)?;
    }

    println!("{:?}", header);

    Ok(())
}
