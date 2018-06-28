bitflags! {
    #[repr(C)]
    pub struct SegmentFlag: u32 {
        const X = 1;
        const W = 2;
        const R = 4;
    }
}
