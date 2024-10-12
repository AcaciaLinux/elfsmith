use std::io::{Read, Seek};

use crate::{
    str_from_null_terminated, Header, Packable, ProgramHeader, SectionHeader, UnpackError,
};

/// A representation of a ELF file
#[derive(Debug)]
pub struct ELFFile {
    /// The header of the ELF file
    pub header: Header,
    /// The program headers
    pub program_headers: Vec<ProgramHeader>,
    /// The section headers
    pub section_headers: Vec<SectionHeader>,
}

impl ELFFile {
    /// Loads a ELF file from the provided stream
    ///
    /// This will **not** load the binary blobs, only headers
    /// # Arguments
    /// * `r` - The stream to read from
    pub fn load<R: Read + Seek>(r: &mut R) -> Result<Self, UnpackError> {
        let header = Header::unpack(r, false)?;

        let program_headers = header.read_program_headers(r)?;
        let section_headers = header.read_section_headers(r)?;

        Ok(Self {
            header,
            program_headers,
            section_headers,
        })
    }

    /// Gets a string by offset from the `.shstrtab` section
    /// # Arguments
    /// * `offset` - The offset into the binary data of the section
    /// # Returns
    /// `None` if the section table is not found or invalid or `offset`
    /// is out of bounds
    pub fn get_sh_string(&self, offset: usize) -> Option<String> {
        let section_names = &self
            .section_headers
            .get(self.header.sh_str_index as usize)?;

        if offset > section_names.size as usize {
            return None;
        }

        let ptr = section_names.data.blob.as_ptr();

        let s = unsafe { str_from_null_terminated(ptr.byte_add(offset)) };

        Some(s.to_owned())
    }
}
