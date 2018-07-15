extern crate elf_loader;

use elf_loader::loader::ElfLoader;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let mut loader = ElfLoader::new(&args[1])?;
    loader.load_memory();
    unsafe {
        use std::mem::transmute;
        let entry: fn() = transmute(loader.get_entry_addr());
        entry();
    }
    Ok(())
}
