pub mod flag;
pub mod prog_header_type;

#[derive(Debug, Clone)]
pub enum ProgramHeaders {
    Header32(Vec<ProgramHeader32>),
    Header64(Vec<ProgramHeader64>),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProgramHeader32 {
    pub prog_header_type: prog_header_type::ProgramHeaderType,
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
    pub prog_header_type: prog_header_type::ProgramHeaderType,
    pub flags: flag::SegmentFlag,
    pub offset: u64,
    pub vert_addr: u64,
    pub phys_addr: u64,
    pub file_size: u64,
    pub mem_size: u64,
    pub align: u64,
}
