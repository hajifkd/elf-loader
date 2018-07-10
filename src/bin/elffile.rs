extern crate elf_loader;

use elf_loader::file::elf_file::ElfFile;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let file = ElfFile::new(&args[1])?;
    println!("{:?}", file.prog_headers);
    println!("{:?}", file.sect_headers);
    println!("{:?}", file.sect_header_names);
    Ok(())
}
