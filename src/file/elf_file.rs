use header::header::*;
use header::prog_header::*;
use header::sect_header::*;
use std;
use std::ffi::CStr;
use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::mem;
use std::slice;

#[derive(Debug)]
pub struct ElfFile {
    pub header: ElfHeader,
    pub file: File,
    pub prog_headers: ProgramHeaders,
    pub sect_headers: SectionHeaders,
    pub sect_header_names: Vec<String>,
}

unsafe fn read_into_ptr(
    file: &mut File,
    ptr: *mut u8,
    size: usize,
    offset: u64,
) -> Result<(), std::io::Error> {
    let slice = slice::from_raw_parts_mut(ptr, size);
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(slice)?;

    Ok(())
}

macro_rules! read_header {
    ($fn_name:ident, $sum_type:ident, $entry_size:ident, $entry_num:ident, $offset:ident) => {
        fn $fn_name(header: &ElfHeader, file: &mut File) -> Result<$sum_type, std::io::Error> {
            let is_32 = header.is_32()?;
            let total_size = header.$entry_size as usize * header.$entry_num as usize;
            let offset = header.$offset as _;

            unsafe {
                if is_32 {
                    let mut v = vec![mem::zeroed(); header.$entry_num as _];
                    read_into_ptr(file, v.as_mut_ptr() as _, total_size, offset)?;
                    Ok($sum_type::Header32(v))
                } else {
                    let mut v = vec![mem::zeroed(); header.$entry_num as _];
                    read_into_ptr(file, v.as_mut_ptr() as _, total_size, offset)?;
                    Ok($sum_type::Header64(v))
                }
            }
        }
    };
}

read_header! {
    read_prog_headers,
    ProgramHeaders,
    prog_header_entry_size,
    prog_header_entry_num,
    prog_header_offset
}

read_header! {
    read_sect_headers,
    SectionHeaders,
    sect_header_entry_size,
    sect_header_entry_num,
    sect_header_offset
}

impl ElfFile {
    pub fn new(path: &str) -> Result<ElfFile, std::io::Error> {
        ElfFile::new_from_file(File::open(path)?)
    }

    pub fn new_from_file(mut file: File) -> Result<ElfFile, std::io::Error> {
        let mut header: ElfHeader = unsafe { mem::zeroed() };
        let header_size = mem::size_of::<ElfHeader>();
        unsafe {
            read_into_ptr(&mut file, &mut header as *mut _ as _, header_size, 0)?;
        }

        let sect_headers = read_sect_headers(&header, &mut file)?;

        macro_rules! read_names {
            ($shdrs:expr) => {{
                let strtab = $shdrs[header.sect_header_name_tbl_index as usize];
                let mut buf = vec![0u8; strtab.size as usize];
                unsafe {
                    read_into_ptr(
                        &mut file,
                        buf.as_mut_ptr() as _,
                        strtab.size as usize,
                        strtab.offset as u64,
                    )?;
                }
                let buf = buf;

                let mut res = $shdrs
                    .iter()
                    .map(|&shdr| {
                        if shdr.sect_type == SectionHeaderType::NULL {
                            Ok("".to_owned())
                        } else {
                            unsafe {
                                Ok(
                                    CStr::from_ptr(&buf[shdr.name_index as usize] as *const u8 as _)
                                        .to_str()?
                                        .to_owned(),
                                )
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                if res.iter()
                    .any(|r: &Result<_, std::str::Utf8Error>| r.is_err())
                {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Cannot decode section header name.",
                    ));
                }

                res.into_iter().map(|r| r.unwrap()).collect()
            }};
        }

        let sect_header_names = match sect_headers {
            SectionHeaders::Header32(ref shdrs) => read_names!(shdrs),
            SectionHeaders::Header64(ref shdrs) => read_names!(shdrs),
        };

        Ok(ElfFile {
            header,
            prog_headers: read_prog_headers(&header, &mut file)?,
            sect_headers,
            sect_header_names,
            file,
        })
    }
}
