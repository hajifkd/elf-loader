use header::header::*;
use header::ident::*;
use header::prog_header::*;
use header::sect_header::*;
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

macro_rules! read_header {
    ($fn_name:ident, $sum_type:ident, $entry_size:ident, $entry_num:ident, $offset: ident) => {
        pub fn $fn_name(&mut self) -> Result<$sum_type, std::io::Error> {
            let is_32 = self.is_32()?;
            let total_size = self.header.$entry_size as usize * self.header.$entry_num as usize;
            let offset = self.header.$offset as _;

            unsafe {
                if is_32 {
                    let mut v = vec![mem::zeroed(); self.header.$entry_num as _];
                    self.read_into_ptr(v.as_mut_ptr() as _, total_size, offset)?;
                    Ok($sum_type::Header32(v))
                } else {
                    let mut v = vec![mem::zeroed(); self.header.$entry_num as _];
                    self.read_into_ptr(v.as_mut_ptr() as _, total_size, offset)?;
                    Ok($sum_type::Header64(v))
                }
            }
        }
    };
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

    fn is_32(&self) -> Result<bool, std::io::Error> {
        match self.header.file_class {
            ElfFileClass::CLASS32 => Ok(true),
            ElfFileClass::CLASS64 => Ok(false),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unknown file class",
                ))
            }
        }
    }

    read_header! {
        prog_headers,
        ProgramHeaders,
        prog_header_entry_size,
        prog_header_entry_num,
        prog_header_offset
    }

    read_header! {
        sect_headers,
        SectionHeaders,
        sect_header_entry_size,
        sect_header_entry_num,
        sect_header_offset
    }
}
