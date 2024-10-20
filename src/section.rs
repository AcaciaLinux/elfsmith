use std::{
    fmt::Debug,
    io::{self},
};

use crate::{Blob, Packable, PackableClass, UnpackError, Unpackable, UnpackableClass};

use super::Class;

/// A section header in the ELF file
#[derive(Debug)]
pub struct SectionHeader {
    /// The index into the `.shstrtab` section for the name of this section
    pub name: u32,
    /// The type of section at hand
    pub ty: u32,
    /// Attributes for this section
    pub flags: u64,
    /// The virtual address for this section
    pub address: u64,
    /// The offset of this section in the file image
    pub offset: u64,
    /// The size in bytes
    pub size: u64,
    /// Section index of an associated section
    pub link: u32,
    /// Additional information about this section
    pub info: u32,
    /// The required alignment of the section - must be a power of two
    pub addr_align: u64,
    /// The size in bytes of fixed-size section entries, otherwise `0`
    pub entry_size: u64,
    /// The section data
    pub data: Blob,
}

impl PackableClass for SectionHeader {
    fn pack_class<W: std::io::Write + io::Seek>(
        self,
        w: &mut W,
        big_endian: bool,
        class: Class,
    ) -> Result<(), std::io::Error> {
        self.name.pack(w, big_endian)?;
        self.ty.pack(w, big_endian)?;

        self.flags.pack_class(w, big_endian, class)?;
        self.address.pack_class(w, big_endian, class)?;
        self.offset.pack_class(w, big_endian, class)?;
        self.size.pack_class(w, big_endian, class)?;
        self.link.pack(w, big_endian)?;
        self.info.pack(w, big_endian)?;
        self.addr_align.pack_class(w, big_endian, class)?;
        self.entry_size.pack_class(w, big_endian, class)?;

        Ok(())
    }
}

impl UnpackableClass for SectionHeader {
    fn unpack_class<R: std::io::Read + io::Seek>(
        r: &mut R,
        big_endian: bool,
        class: Class,
    ) -> Result<Self, UnpackError> {
        let name = u32::unpack(r, big_endian)?;
        let ty = u32::unpack(r, big_endian)?;

        let flags = u64::unpack_class(r, big_endian, class)?;
        let address = u64::unpack_class(r, big_endian, class)?;
        let offset = u64::unpack_class(r, big_endian, class)?;
        let size = u64::unpack_class(r, big_endian, class)?;
        let link = u32::unpack(r, big_endian)?;
        let info = u32::unpack(r, big_endian)?;
        let addr_align = u64::unpack_class(r, big_endian, class)?;
        let entry_size = u64::unpack_class(r, big_endian, class)?;

        let data = Blob::load(r, offset, size as usize)?;

        Ok(Self {
            name,
            ty,
            flags,
            address,
            offset,
            size,
            link,
            info,
            addr_align,
            entry_size,
            data,
        })
    }
}
