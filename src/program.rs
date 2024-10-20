use std::{fmt::Debug, io};

use crate::{Blob, Packable, PackableClass, UnpackError, Unpackable, UnpackableClass};

use super::Class;

/// A program header in the ELF file
#[derive(Debug)]
pub struct ProgramHeader {
    /// The type of segment at hand
    pub ty: u32,
    /// The flags for this segment:
    /// - `0x01`: Executable
    /// - `0x02`: Writable
    /// - `0x04`: Readable
    pub flags: u32,
    /// The offset of the segment in the file image
    pub offset: u64,
    /// The virtual address of this segment in memory
    pub virtual_addr: u64,
    /// The physical address of this segment in memory (if required)
    pub physical_addr: u64,
    /// The size of the segment in the file
    pub file_size: u64,
    /// THe size of the segment in memory
    pub mem_size: u64,
    /// Alignment for this segment, `0` or `1` mean no alignment
    pub alignment: u64,
    /// The program data
    pub data: Blob,
}

impl PackableClass for ProgramHeader {
    fn pack_class<W: std::io::Write + io::Seek>(
        self,
        w: &mut W,
        big_endian: bool,
        class: super::Class,
    ) -> Result<(), std::io::Error> {
        self.ty.pack(w, big_endian)?;

        if class == Class::ELF64 {
            self.flags.pack(w, big_endian)?;
        };

        self.offset.pack_class(w, big_endian, class)?;
        self.virtual_addr.pack_class(w, big_endian, class)?;
        self.physical_addr.pack_class(w, big_endian, class)?;
        self.file_size.pack_class(w, big_endian, class)?;
        self.mem_size.pack_class(w, big_endian, class)?;

        if class == Class::ELF32 {
            self.flags.pack(w, big_endian)?;
        }

        self.alignment.pack_class(w, big_endian, class)?;

        Ok(())
    }
}

impl UnpackableClass for ProgramHeader {
    fn unpack_class<R: std::io::Read + io::Seek>(
        r: &mut R,
        big_endian: bool,
        class: super::Class,
    ) -> Result<Self, UnpackError> {
        let ty = u32::unpack(r, big_endian)?;

        let flags = if class == Class::ELF64 {
            u32::unpack(r, big_endian)?
        } else {
            0
        };

        let offset = u64::unpack_class(r, big_endian, class)?;
        let virtual_addr = u64::unpack_class(r, big_endian, class)?;
        let physical_addr = u64::unpack_class(r, big_endian, class)?;
        let file_size = u64::unpack_class(r, big_endian, class)?;
        let mem_size = u64::unpack_class(r, big_endian, class)?;

        let flags = if class == Class::ELF32 {
            u32::unpack(r, big_endian)?
        } else {
            flags
        };

        let alignment = u64::unpack_class(r, big_endian, class)?;

        let data = Blob::load(r, offset, file_size as usize)?;

        Ok(Self {
            ty,
            flags,
            offset,
            virtual_addr,
            physical_addr,
            file_size,
            mem_size,
            alignment,
            data,
        })
    }
}
