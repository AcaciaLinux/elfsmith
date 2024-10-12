use std::io;

use crate::{Packable, UnpackError};

/// The ident padding sequence, filled with `0`
const ELF_IDENT_PADDING: [u8; 7] = [0u8; 7];

/// The ELF file magic - 0x7f followed by `ELF` in ASCII
pub const ELF_FILE_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

/// The ELF ident structure to identify further
/// parsing of an ELF file
#[derive(Debug)]
pub struct ELFIdent {
    /// The `ELF` file magic [ELF_FILE_MAGIC]
    pub magic: [u8; 4],
    /// The class of the ELF file at hand
    pub class: ELFIdentClass,
    /// The endianness of the file
    pub endianness: ELFIdentEndianness,
    /// The file version (normally `1`)
    pub version: u8,
    /// The operating system ABI
    pub os_abi: u8,
    /// The abi version
    pub abi_version: u8,
}

/// The class of the ELF file at hand
#[derive(Debug)]
pub enum ELFIdentClass {
    /// A `32` bit ELF file
    ELF32 = 1,
    /// A `64` bit ELF file
    ELF64 = 2,
}

/// The endianness of this file
#[derive(Debug)]
pub enum ELFIdentEndianness {
    /// Little endian
    Little = 1,
    /// Big endian
    Big = 2,
}

impl Packable for ELFIdentClass {
    fn pack<W: io::Write>(self, w: &mut W, _: bool) -> Result<(), io::Error> {
        w.write_all(&[self as u8])
    }

    fn unpack<R: io::Read>(r: &mut R, _: bool) -> Result<Self, UnpackError> {
        let mut data = [0u8];

        r.read_exact(&mut data)?;

        match data[0] {
            1 => Ok(Self::ELF32),
            2 => Ok(Self::ELF64),
            _ => Err(UnpackError::InvalidEnumVariant {
                name: "ELFIdentClass".into(),
                variant: data[0] as usize,
            }),
        }
    }
}

impl Packable for ELFIdentEndianness {
    fn pack<W: io::Write>(self, w: &mut W, _: bool) -> Result<(), io::Error> {
        w.write_all(&[self as u8])
    }

    fn unpack<R: io::Read>(r: &mut R, _: bool) -> Result<Self, UnpackError> {
        let mut data = [0u8];

        r.read_exact(&mut data)?;

        match data[0] {
            1 => Ok(Self::Little),
            2 => Ok(Self::Big),
            _ => Err(UnpackError::InvalidEnumVariant {
                name: "ELFIdentEndianness".into(),
                variant: data[0] as usize,
            }),
        }
    }
}

impl Packable for ELFIdent {
    fn pack<W: io::Write>(self, w: &mut W, _: bool) -> Result<(), io::Error> {
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

    fn unpack<R: io::Read>(r: &mut R, _: bool) -> Result<Self, UnpackError> {
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
            class: ELFIdentClass::unpack(r, false)?,
            endianness: ELFIdentEndianness::unpack(r, false)?,
            version: u8::unpack(r, false)?,
            os_abi: u8::unpack(r, false)?,
            abi_version: u8::unpack(r, false)?,
        };

        let mut padding = ELF_IDENT_PADDING;
        r.read_exact(&mut padding)?;

        Ok(s)
    }
}
