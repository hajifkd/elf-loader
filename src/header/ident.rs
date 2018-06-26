#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ElfMagic {
    Magic0 = 0x7f,
    Magic1 = 'E' as _,
    Magic2 = 'L' as _,
    Magic3 = 'F' as _,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ElfFileClass {
    None = 0,
    Class32 = 1,
    Class64 = 2,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ElfDataEncoding {
    None = 0,
    LittleEndian = 1,
    BigEndian = 2,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ElfVersion {
    None = 0,
    Current = 1,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ElfOSABI {
    Sysv = 0,
    HPUX = 1,
    NetBSD = 2,
    Linux = 3,
    Hurd = 4,
    _86Open = 5,
    Solaris = 6,
    Monterey = 7,
    IRIX = 8,
    FreeBSD = 9,
    TRU64 = 10,
    Modesto = 11,
    OpenBSD = 12,
    ARM = 97,
    StandAlone = 255,
}
