use std::io::{Read, Seek};

use crate::{Header, Packable, ProgramHeader, SectionHeader, UnpackError};

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

    /// Loads a ELF file fully from the provided stream
    ///
    /// This will load the **ALL** binary blobs, only headers
    /// # Arguments
    /// * `r` - The stream to read from
    pub fn load_fully<R: Read + Seek>(r: &mut R) -> Result<Self, UnpackError> {
        let header = Header::unpack(r, false)?;

        let mut program_headers = header.read_program_headers(r)?;
        for header in &mut program_headers {
            header.load(r)?;
        }

        let mut section_headers = header.read_section_headers(r)?;
        for header in &mut section_headers {
            header.load(r)?;
        }

        Ok(Self {
            header,
            program_headers,
            section_headers,
        })
    }
}
