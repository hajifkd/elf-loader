#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ProgramHeaderType {
    Null = 0,
    Loadable = 1,
    DynamicLinking = 2,
    Interpreter = 3,
    Note = 4,
    Shlib = 5,
    ProgramHeader = 6,
    ThreadLocalStroage = 7,

    GnuEHFrame = 0x6474e550,
    GnuStack = 0x6474e551,
    GnuRelro = 0x6474e552,
}
