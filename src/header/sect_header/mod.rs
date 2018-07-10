pub mod flag;
pub mod sect_header_type;

pub use self::flag::SectionFlag32;
pub use self::flag::SectionFlag64;
pub use self::sect_header_type::SectionHeaderType;

#[derive(Debug, Clone)]
pub enum SectionHeaders {
    Header32(Vec<SectionHeader32>),
    Header64(Vec<SectionHeader64>),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SectionHeader32 {
    pub name_index: u32,
    pub sect_type: sect_header_type::SectionHeaderType,
    pub flags: flag::SectionFlag32,
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
    pub sect_type: sect_header_type::SectionHeaderType,
    pub flags: flag::SectionFlag64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub addr_aligh: u64,
    pub entry_size: u64,
}
