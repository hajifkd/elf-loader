use aligned_alloc;
use file::ElfFile;
use header::{ElfMagic, ElfType, ElfVersion, ProgramHeaders};
use std;
use util::read_into_ptr;

pub struct ElfLoader {
    elf_file: ElfFile,
    memory: Option<*mut u8>,
}

impl Drop for ElfLoader {
    fn drop(&mut self) {
        if let Some(ptr) = self.memory {
            unsafe {
                aligned_alloc::aligned_free(ptr as _);
            }
        }

        self.memory = None;
    }
}

impl ElfLoader {
    pub fn new(path: &str) -> Result<ElfLoader, std::io::Error> {
        let res = ElfLoader {
            elf_file: ElfFile::new(path)?,
            memory: None,
        };

        res.sanity_check();

        Ok(res)
    }

    // TODO probably use failure?
    fn sanity_check(&self) {
        if self.elf_file.header.magics
            != [
                ElfMagic::MAGIC0,
                ElfMagic::MAGIC1,
                ElfMagic::MAGIC2,
                ElfMagic::MAGIC3,
            ] || self.elf_file.header.elf_version != ElfVersion::CURRENT
        {
            panic!("Invalid Elf File");
        }

        if self.elf_file.header.elf_type != ElfType::SHARED_OBJECT {
            panic!("Only PIE binaries are supported.");
        }
    }

    fn allocate_memory(&mut self) {
        macro_rules! max_addr {
            ($phdrs:expr) => {
                $phdrs
                    .iter()
                    .map(|&h| h.vert_addr + h.mem_size)
                    .max()
                    .unwrap() as usize
            };
        }

        let max_addr = match self.elf_file.prog_headers {
            ProgramHeaders::Header32(ref phdrs) => max_addr!(phdrs),
            ProgramHeaders::Header64(ref phdrs) => max_addr!(phdrs),
        };

        const PAGE_SIZE: usize = 0x1000;

        // No virtual address...
        let memory = aligned_alloc::aligned_alloc(max_addr, PAGE_SIZE) as *mut u8;

        self.memory = Some(memory);
    }

    fn apply_mprotect(&mut self) {
        self.allocate_memory();

        macro_rules! mprotect {
            ($phdrs:expr) => {
                for phdr in $phdrs {
                    unsafe {
                        ::util::mprotect(
                            self.memory.unwrap().add(phdr.offset as usize),
                            phdr.mem_size as usize,
                            phdr.flags.to_mprotect_flags(),
                        );
                    }
                }
            };
        }

        match self.elf_file.prog_headers {
            ProgramHeaders::Header64(ref phdrs) => mprotect!(phdrs),
            ProgramHeaders::Header32(ref phdrs) => mprotect!(phdrs),
        }
    }

    pub fn load_memory(&mut self) {
        if self.memory.is_some() {
            return;
        }

        self.allocate_memory();

        macro_rules! load_memory {
            ($phdrs:expr) => {
                for phdr in $phdrs {
                    unsafe {
                        read_into_ptr(
                            &mut self.elf_file.file,
                            self.memory.unwrap().add(phdr.vert_addr as usize),
                            phdr.file_size as usize,
                            phdr.offset as u64,
                        ).unwrap();
                    }
                }
            };
        }

        match self.elf_file.prog_headers {
            ProgramHeaders::Header64(ref phdrs) => load_memory!(phdrs),
            ProgramHeaders::Header32(ref phdrs) => load_memory!(phdrs),
        }

        self.apply_mprotect();
    }
}
