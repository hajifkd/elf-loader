cenums! {
    #[repr(C)]
    pub struct ElfType: u16 {
        const NONE = 0;
        const RELOCATABLE = 1;
        const EXECUTABLE = 2;
        const SHARED_OBJECT = 3;
        const CORE = 4;
    }
}
