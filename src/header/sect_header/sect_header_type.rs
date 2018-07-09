cenums! {
    #[repr(C)]
    pub struct SectionHeaderType: u32 {
        const NULL = 0;                         // No associated section (inactive entry).
        const PROGBITS = 1;                     // Program-defined contents.
        const SYMBOL_TABLE = 2;                 // Symbol table.
        const STRING_TABLE = 3;                 // String table.
        const RELA = 4;                         // Relocation entries; explicit addends.
        const HASH = 5;                         // Symbol hash table.
        const DYNAMIC_LINKING = 6;              // Information for dynamic linking.
        const NOTE = 7;                         // Information about the file.
        const NOBITS = 8;                       // Data occupies no space in the file.
        const REL = 9;                          // Relocation entries; no explicit addends.
        const SHLIB = 10;                       // Reserved.
        const DYNSYM = 11;                      // Symbol table.
        const INIT_ARRAY = 14;                  // Pointers to initialization functions.
        const FINI_ARRAY = 15;                  // Pointers to termination functions.
        const PREINIT_ARRAY = 16;               // Pointers to pre-init functions.
        const GROUP = 17;                       // Section group.
        const SYMTAB_SHNDX = 18;                // Indices for SHN_XINDEX entries.
        // Experimental support for SHT_RELR sections. For details, see proposal
        // at https://groups.google.com/forum/#!topic/generic-abi/bX460iggiKg
        const RELR = 19;                        // Relocation entries; only offsets.
        const LOOS = 0x60000000;                // Lowest operating system-specific type.
        // Android packed relocation section types.
        // https://android.googlesource.com/platform/bionic/+/6f12bfece5dcc01325e0abba56a46b1bcf991c69/tools/relocation_packer/src/elf_file.cc#37
        const ANDROID_REL = 0x60000001;
        const ANDROID_RELA = 0x60000002;
        const LLVM_ODRTAB = 0x6fff4c00;         // LLVM ODR table.
        const LLVM_LINKER_OPTIONS = 0x6fff4c01; // LLVM Linker Options.
        const LLVM_CALL_GRAPH_PROFILE = 0x6fff4c02; // LLVM Call Graph Profile.

        // Android's experimental support for SHT_RELR sections.
        // https://android.googlesource.com/platform/bionic/+/b7feec74547f84559a1467aca02708ff61346d2a/libc/include/elf.h#512
        const ANDROID_RELR = 0x6fffff00;        // Relocation entries; only offsets.
        const GNU_ATTRIBUTES = 0x6ffffff5;      // Object attributes.
        const GNU_HASH = 0x6ffffff6;            // GNU-style hash table.
        const GNU_VERDEF = 0x6ffffffd;          // GNU version definitions.
        const GNU_VERNEED = 0x6ffffffe;         // GNU version references.
        const GNU_VERSYM = 0x6fffffff;          // GNU symbol versions table.
        const HIOS = 0x6fffffff;                // Highest operating system-specific type.
        const LOPROC = 0x70000000;              // Lowest processor arch-specific type.

        // Fixme: All this is duplicated in MCSectionELF. Why??
        // Exception Index table
        const ARM_EXIDX = 0x70000001;
        // BPABI DLL dynamic linking pre-emption map
        const ARM_PREEMPTMAP = 0x70000002;
        //  Object file compatibility attributes
        const ARM_ATTRIBUTES = 0x70000003;
        const ARM_DEBUGOVERLAY = 0x70000004;
        const ARM_OVERLAYSECTION = 0x70000005;
        const HEX_ORDERED = 0x70000000;         // Link editor is to sort the entries in
                                                // this section based on their sizes
        const X86_64_UNWIND = 0x70000001;       // Unwind information
        const MIPS_REGINFO = 0x70000006;        // Register usage information
        const MIPS_OPTIONS = 0x7000000d;        // General options
        const MIPS_DWARF = 0x7000001e;          // DWARF debugging section.
        const MIPS_ABIFLAGS = 0x7000002a;       // ABI information.
        const HIPROC = 0x7fffffff;              // Highest processor arch-specific type.
        const LOUSER = 0x80000000;              // Lowest type reserved for applications.
        const HIUSER = 0xffffffff;              // Highest type reserved for applications.
    }
}
