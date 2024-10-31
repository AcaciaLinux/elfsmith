use std::io;

use crate::{Packable, UnpackError, Unpackable};

/// The ident padding sequence, filled with `0`
const ELF_IDENT_PADDING: [u8; 7] = [0u8; 7];

/// The ELF file magic - 0x7f followed by `ELF` in ASCII
pub const ELF_FILE_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

/// The ELF ident structure to identify further
/// parsing of an ELF file
#[derive(Debug)]
pub struct Ident {
    /// The `ELF` file magic [ELF_FILE_MAGIC]
    pub magic: [u8; 4],
    /// The class of the ELF file at hand
    pub class: Class,
    /// The endianness of the file
    pub endianness: Endianness,
    /// The file version (normally `1`)
    pub version: u8,
    /// The operating system ABI
    pub os_abi: u8,
    /// The abi version
    pub abi_version: u8,
}

/// The class of the ELF file at hand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    /// A `32` bit ELF file
    ELF32 = 1,
    /// A `64` bit ELF file
    ELF64 = 2,
}

/// The endianness of this file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    /// Little endian
    Little = 1,
    /// Big endian
    Big = 2,
}

impl Ident {
    /// Returns whether the ident describes
    /// the file to be big endian
    pub fn is_big_endian(&self) -> bool {
        self.endianness == Endianness::Big
    }
}

impl Packable for Class {
    fn pack<W: io::Write + io::Seek>(&self, w: &mut W, _: bool) -> Result<(), io::Error> {
        w.write_all(&[*self as u8])
    }
}

impl Unpackable for Class {
    fn unpack<R: io::Read>(r: &mut R, _: bool) -> Result<Self, UnpackError> {
        let mut data = [0u8];

        r.read_exact(&mut data)?;

        match data[0] {
            1 => Ok(Self::ELF32),
            2 => Ok(Self::ELF64),
            _ => Err(UnpackError::InvalidEnumVariant {
                name: "Class".into(),
                variant: data[0] as usize,
            }),
        }
    }
}

impl Packable for Endianness {
    fn pack<W: io::Write + io::Seek>(&self, w: &mut W, _: bool) -> Result<(), io::Error> {
        w.write_all(&[*self as u8])
    }
}

impl Unpackable for Endianness {
    fn unpack<R: io::Read>(r: &mut R, _: bool) -> Result<Self, UnpackError> {
        let mut data = [0u8];

        r.read_exact(&mut data)?;

        match data[0] {
            1 => Ok(Self::Little),
            2 => Ok(Self::Big),
            _ => Err(UnpackError::InvalidEnumVariant {
                name: "Endianness".into(),
                variant: data[0] as usize,
            }),
        }
    }
}

impl Packable for Ident {
    fn pack<W: io::Write + io::Seek>(&self, w: &mut W, _: bool) -> Result<(), io::Error> {
        w.write_all(&self.magic)?;

        w.write_all(&[
            self.class as u8,
            self.endianness as u8,
            self.version,
            self.os_abi,
            self.abi_version,
        ])?;

        w.write_all(&ELF_IDENT_PADDING)
    }
}

impl Unpackable for Ident {
    fn unpack<R: io::Read + io::Seek>(r: &mut R, _: bool) -> Result<Self, UnpackError> {
        let mut magic = [0u8; 4];
        r.read_exact(&mut magic)?;

        if magic != ELF_FILE_MAGIC {
            return Err(UnpackError::InvalidMagic {
                expected: ELF_FILE_MAGIC.into(),
                got: magic.into(),
            });
        }

        let s = Self {
            magic,
            class: Class::unpack(r, false)?,
            endianness: Endianness::unpack(r, false)?,
            version: u8::unpack(r, false)?,
            os_abi: u8::unpack(r, false)?,
            abi_version: u8::unpack(r, false)?,
        };

        let mut padding = ELF_IDENT_PADDING;
        r.read_exact(&mut padding)?;

        Ok(s)
    }
}
