use header::header::*;
use std;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::slice;

#[derive(Debug)]
pub struct ElfFile {
    header: ElfHeader,
    file: File,
}

impl ElfFile {
    pub fn new(mut file: File) -> Result<ElfFile, std::io::Error> {
        let mut header: ElfHeader = unsafe { mem::zeroed() };
        let header_size = mem::size_of::<ElfHeader>();
        unsafe {
            let header_slice =
                slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, header_size);
            file.read_exact(header_slice)?;
        }

        Ok(ElfFile { header, file })
    }

    pub fn prog_headers(&mut self) -> Result<ProgramHeaders, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "a"))
    }
}
