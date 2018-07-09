bitflags! {
    #[repr(C)]
    pub struct SectionFlag32: u32 {
        const WRITE = 0x1;
        const ALLOC = 0x2;
        const EXECINSTR = 0x4;
    }
}

bitflags! {
    #[repr(C)]
    pub struct SectionFlag64: u32 {
        const WRITE = 0x1;
        const ALLOC = 0x2;
        const EXECINSTR = 0x4;
    }
}
