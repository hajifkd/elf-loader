use super::elf_type;
use super::ident;
use super::machine;
use super::prog_head::flag;
use super::prog_head::prog_head_type;

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
    pub prog_head_offset: usize,
    pub sect_head_offset: usize,
    pub flags: u32, // not used
    pub elf_header_size: u16,
    pub prog_head_entry_size: u16,
    pub prog_head_entry_num: u16,
    pub sect_head_entry_size: u16,
    pub sect_head_entry_num: u16,
    pub sect_head_tbl_index: u16,
}

#[derive(Debug, Clone)]
pub enum ProgramHeaders {
    Header32(Vec<ProgramHeader32>),
    Header64(Vec<ProgramHeader64>),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProgramHeader32 {
    pub prog_head_type: prog_head_type::ProgramHeaderType,
    pub offset: u32,
    pub vert_addr: u32,
    pub phys_addr: u32,
    pub file_size: u32,
    pub mem_size: u32,
    pub flags: flag::SegmentFlag,
    pub align: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProgramHeader64 {
    pub prog_head_type: prog_head_type::ProgramHeaderType,
    pub flags: flag::SegmentFlag,
    pub offset: u64,
    pub vert_addr: u64,
    pub phys_addr: u64,
    pub file_size: u64,
    pub mem_size: u64,
    pub align: u64,
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
