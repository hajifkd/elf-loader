use header::header::*;
use header::ident::*;
use header::prog_header::*;
use std;
use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::mem;
use std::slice;

#[derive(Debug)]
pub struct ElfFile {
    header: ElfHeader,
    file: File,
}

impl ElfFile {
    pub fn new(path: &str) -> Result<ElfFile, std::io::Error> {
        ElfFile::new_from_file(File::open(path)?)
    }

    pub fn new_from_file(mut file: File) -> Result<ElfFile, std::io::Error> {
        let mut header: ElfHeader = unsafe { mem::zeroed() };
        let header_size = mem::size_of::<ElfHeader>();
        unsafe {
            let header_slice =
                slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, header_size);
            file.read_exact(header_slice)?;
        }

        Ok(ElfFile { header, file })
    }

    unsafe fn read_into_ptr(
        &mut self,
        ptr: *mut u8,
        size: usize,
        offset: u64,
    ) -> Result<(), std::io::Error> {
        let slice = slice::from_raw_parts_mut(ptr, size);
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(slice)?;

        Ok(())
    }

    pub fn prog_headers(&mut self) -> Result<ProgramHeaders, std::io::Error> {
        let is_32 = match self.header.file_class {
            ElfFileClass::CLASS32 => true,
            ElfFileClass::CLASS64 => false,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unknown file class",
                ))
            }
        };

        let header_size = if is_32 {
            mem::size_of::<ProgramHeader32>()
        } else {
            mem::size_of::<ProgramHeader64>()
        };

        assert_eq!(header_size, self.header.prog_header_entry_size as _);
        let total_size = header_size * self.header.prog_header_entry_num as usize;
        let offset = self.header.prog_header_offset as _;

        unsafe {
            if is_32 {
                let mut v =
                    vec![mem::zeroed::<ProgramHeader32>(); self.header.prog_header_entry_num as _];
                self.read_into_ptr(v.as_mut_ptr() as _, total_size, offset)?;
                Ok(ProgramHeaders::Header32(v))
            } else {
                let mut v =
                    vec![mem::zeroed::<ProgramHeader64>(); self.header.prog_header_entry_num as _];
                self.read_into_ptr(v.as_mut_ptr() as _, total_size, offset)?;
                Ok(ProgramHeaders::Header64(v))
            }
        }
    }
}
