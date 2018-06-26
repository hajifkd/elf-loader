#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum ElfType {
    None = 0,
    Relocatable = 1,
    Executable = 2,
    SharedObject = 3,
    Core = 4,
}
