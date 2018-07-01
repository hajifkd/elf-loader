cenums! {
    #[repr(C)]
    pub struct ElfMagic: u8 {
        const MAGIC0 = 0x7f;
        const MAGIC1 = 'E' as _;
        const MAGIC2 = 'L' as _;
        const MAGIC3 = 'F' as _;
    }
}

cenums! {
    #[repr(C)]
    pub struct ElfFileClass: u8 {
        const NONE = 0;
        const CLASS32 = 1;
        const CLASS64 = 2;
    }
}

cenums! {
    #[repr(C)]
    pub struct ElfDataEncoding: u8 {
        const NONE = 0;
        const LITTLE_ENDIAN = 1;
        const BIG_ENDIAN = 2;
    }
}

cenums! {
    #[repr(C)]
    pub struct ElfVersion: u8 {
        const NONE = 0;
        const CURRENT = 1;
    }
}

cenums! {
    #[repr(C)]
    pub struct ElfOsAbi: u8 {
        const SYSV = 0;
        const HPUX = 1;
        const NETBSD = 2;
        const LINUX = 3;
        const HURD = 4;
        const _86OPEN = 5;
        const SOLARIS = 6;
        const MONTEREY = 7;
        const IRIX = 8;
        const FREEBSD = 9;
        const TRU64 = 10;
        const MODESTO = 11;
        const OPENBSD = 12;
        const ARM = 97;
        const STANDALONE = 255;
    }
}
