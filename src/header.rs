use std::io::{Read, Seek, SeekFrom};

use crate::{Packable, PackableClass, UnpackError, Unpackable, UnpackableClass};

use super::{Ident, ProgramHeader, SectionHeader};

const ELF_HEADER_SIZE_32: u64 = 0x34;
const ELF_HEADER_SIZE_64: u64 = 0x40;

const PROGRAM_HEADER_SIZE_32: u64 = 0x20;
const PROGRAM_HEADER_SIZE_64: u64 = 0x38;

const SECTION_HEADER_SIZE_32: u64 = 0x28;
const SECTION_HEADER_SIZE_64: u64 = 0x40;

/// The ELF header
#[derive(Debug)]
pub struct Header {
    /// The ident sequence
    pub ident: Ident,
    /// The type of ELF file
    pub ty: u16,
    /// The machine type
    pub machine: u16,
    /// The file version (`1`)
    pub version: u32,
    /// The entry point of the file, if existing
    pub entry_point: u64,
    /// The offset for the program headers
    pub ph_offset: u64,
    /// The offset for the section headers
    pub sh_offset: u64,
    /// Target architecture dependent flags
    pub flags: u32,
    /// The size of this header
    header_size: u16,
    /// The size of a program header
    ph_entry_size: u16,
    /// The count of program headers
    ph_entry_count: u16,
    /// The size of a section header
    sh_entry_size: u16,
    /// The count of section headers
    sh_entry_count: u16,
    /// The section header index of the string table
    pub sh_str_index: u16,
}

impl Header {
    /// Reads the program headers
    /// # Arguments
    /// * `r` - The reader to read the headers from
    pub fn read_program_headers<R: Read + Seek>(
        &self,
        r: &mut R,
    ) -> Result<Vec<ProgramHeader>, UnpackError> {
        r.seek(SeekFrom::Start(self.ph_offset))?;

        let mut res = Vec::new();

        for _ in 0..self.ph_entry_count {
            res.push(ProgramHeader::unpack_class(
                r,
                self.ident.is_big_endian(),
                self.ident.class,
            )?)
        }

        Ok(res)
    }

    /// Reads the section headers
    /// # Arguments
    /// * `r` - The reader to read the headers from
    pub fn read_section_headers<R: Read + Seek>(
        &self,
        r: &mut R,
    ) -> Result<Vec<SectionHeader>, UnpackError> {
        r.seek(SeekFrom::Start(self.sh_offset))?;

        let mut res = Vec::new();

        for _ in 0..self.sh_entry_count {
            res.push(SectionHeader::unpack_class(
                r,
                self.ident.is_big_endian(),
                self.ident.class,
            )?)
        }

        Ok(res)
    }

    /// Returns the size of this header in the file
    ///
    /// This is derived from the ELF class (32 / 64 bit)
    pub fn get_header_size(&self) -> u64 {
        match self.ident.class {
            crate::Class::ELF32 => ELF_HEADER_SIZE_32,
            crate::Class::ELF64 => ELF_HEADER_SIZE_64,
        }
    }

    /// Returns the size a program header takes as described by this header
    ///
    /// This is derived from the ELF class (32 / 64 bit)
    pub fn get_program_header_size(&self) -> u64 {
        match self.ident.class {
            crate::Class::ELF32 => PROGRAM_HEADER_SIZE_32,
            crate::Class::ELF64 => PROGRAM_HEADER_SIZE_64,
        }
    }

    /// Returns the size a section header takes as described by this header
    ///
    /// This is derived from the ELF class (32 / 64 bit)
    pub fn get_section_header_size(&self) -> u64 {
        match self.ident.class {
            crate::Class::ELF32 => SECTION_HEADER_SIZE_32,
            crate::Class::ELF64 => SECTION_HEADER_SIZE_64,
        }
    }
}

impl Packable for Header {
    fn pack<W: std::io::Write + std::io::Seek>(
        &self,
        w: &mut W,
        _: bool,
    ) -> Result<(), std::io::Error> {
        let big_endian = self.ident.is_big_endian();
        let class = self.ident.class;

        self.ident.pack(w, big_endian)?;

        self.ty.pack(w, big_endian)?;
        self.machine.pack(w, big_endian)?;
        self.version.pack(w, big_endian)?;

        self.entry_point.pack_class(w, big_endian, class)?;
        self.ph_offset.pack_class(w, big_endian, class)?;
        self.sh_offset.pack_class(w, big_endian, class)?;

        self.flags.pack(w, big_endian)?;
        self.header_size.pack(w, big_endian)?;

        self.ph_entry_size.pack(w, big_endian)?;
        self.ph_entry_count.pack(w, big_endian)?;
        self.sh_entry_size.pack(w, big_endian)?;
        self.sh_entry_count.pack(w, big_endian)?;

        self.sh_str_index.pack(w, big_endian)?;

        Ok(())
    }
}

impl Unpackable for Header {
    fn unpack<R: std::io::Read + std::io::Seek>(
        r: &mut R,
        _: bool,
    ) -> Result<Self, crate::UnpackError> {
        let ident = Ident::unpack(r, false)?;

        let big_endian = ident.is_big_endian();
        let class = ident.class;

        let ty = u16::unpack(r, big_endian)?;
        let machine = u16::unpack(r, big_endian)?;
        let version = u32::unpack(r, big_endian)?;

        let entry_point = u64::unpack_class(r, big_endian, class)?;
        let ph_offset = u64::unpack_class(r, big_endian, class)?;
        let sh_offset = u64::unpack_class(r, big_endian, class)?;

        let flags = u32::unpack(r, big_endian)?;
        let header_size = u16::unpack(r, big_endian)?;

        let ph_entry_size = u16::unpack(r, big_endian)?;
        let ph_entry_count = u16::unpack(r, big_endian)?;
        let sh_entry_size = u16::unpack(r, big_endian)?;
        let sh_entry_count = u16::unpack(r, big_endian)?;

        let sh_str_index = u16::unpack(r, big_endian)?;

        Ok(Self {
            ident,
            ty,
            machine,
            version,
            entry_point,
            ph_offset,
            sh_offset,
            flags,
            header_size,
            ph_entry_size,
            ph_entry_count,
            sh_entry_size,
            sh_entry_count,
            sh_str_index,
        })
    }
}
