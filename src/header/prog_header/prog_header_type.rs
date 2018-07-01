cenums! {
    #[repr(C)]
    pub struct ProgramHeaderType: u32 {
        const NULL = 0;
        const LOADABLE = 1;
        const DYNAMIC_LINKING = 2;
        const INTERPRETER = 3;
        const NOTE = 4;
        const SHLIB = 5;
        const PROGRAM_HEADER = 6;
        const THREAD_LOCAL_STORAGE = 7;

        const GNU_EH_FRAME = 0x6474e550;
        const GNU_STACK = 0x6474e551;
        const GNU_RELRO = 0x6474e552;
    }
}
