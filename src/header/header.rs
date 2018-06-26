use super::elf_type;
use super::ident;
use super::machine;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ElfHeader {
    magics: [ident::ElfMagic; 4],
    file_class: ident::ElfFileClass,
    data_encoding: ident::ElfDataEncoding,
    elf_version: ident::ElfVersion,
    os_abi: ident::ElfOSABI,
    abi_version: u8,
    pad: [u8; 7], // ident
    elf_type: elf_type::ElfType,
    machine: machine::ElfMachine,
    version: u32,
    entry: usize,
    prog_head_offset: usize,
    sect_head_offset: usize,
    flags: u32, // not used
    elf_header_size: u16,
    prog_head_entry_size: u16,
    prog_head_entry_num: u16,
    sect_head_entry_size: u16,
    sect_head_entry_num: u16,
    sect_head_tbl_index: u16,
}
