use super::elf_type;
use super::ident;
use super::machine;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ElfHeader {
    pub magics: [ident::ElfMagic; 4],
    pub file_class: ident::ElfFileClass,
    pub data_encoding: ident::ElfDataEncoding,
    pub elf_version: ident::ElfVersion,
    pub os_abi: ident::ElfOSABI,
    pub abi_version: u8,
    pub pad: [u8; 7], // ident
    pub elf_type: elf_type::ElfType,
    pub machine: machine::ElfMachine,
    pub version: u32,
    pub entry: usize,
    pub prog_header_offset: usize,
    pub sect_header_offset: usize,
    pub flags: u32, // not used
    pub elf_header_size: u16,
    pub prog_header_entry_size: u16,
    pub prog_header_entry_num: u16,
    pub sect_header_entry_size: u16,
    pub sect_header_entry_num: u16,
    pub sect_header_tbl_index: u16,
}

#[derive(Debug, Clone)]
pub enum SectionHeaders {
    Header32(Vec<SectionHeader32>),
    Header64(Vec<SectionHeader64>),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SectionHeader32 {
    pub name_index: u32,
    pub sect_type: u32,
    pub flags: u32,
    pub addr: u32,
    pub offset: u32,
    pub size: u32,
    pub link: u32,
    pub info: u32,
    pub addr_aligh: u32,
    pub entry_size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SectionHeader64 {
    pub name_index: u32,
    pub sect_type: u32,
    pub flags: u64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub addr_aligh: u64,
    pub entry_size: u64,
}
