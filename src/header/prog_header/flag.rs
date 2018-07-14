bitflags! {
    #[repr(C)]
    pub struct SegmentFlag: u32 {
        const X = 1;
        const W = 2;
        const R = 4;
    }
}

impl SegmentFlag {
    pub(crate) fn to_mprotect_flags(&self) -> ::util::Prot::Ty {
        use util::Prot::*;

        let table = &[
            (SegmentFlag::X, Execute),
            #[cfg(unix)]
            (SegmentFlag::W, WriteOnly),
            (SegmentFlag::R, ReadOnly),
            (SegmentFlag::X | SegmentFlag::R, ReadExec),
            #[cfg(unix)]
            (SegmentFlag::X | SegmentFlag::W, WriteExec),
            (SegmentFlag::W | SegmentFlag::R, ReadWrite),
            (
                SegmentFlag::X | SegmentFlag::R | SegmentFlag::W,
                ReadWriteExec,
            ),
        ];

        table
            .iter()
            .filter_map(|&(f, v)| if *self == f { Some(v) } else { None })
            .next()
            .unwrap_or(NoAccess)
    }
}
