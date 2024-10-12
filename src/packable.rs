use std::io::{self};

/// An error while unpackin
#[derive(Debug)]
pub enum UnpackError {
    /// An invalid magic sequence was unpacked
    InvalidMagic {
        /// The expected sequence
        expected: Vec<u8>,
        /// The unpacked sequence
        got: Vec<u8>,
    },
    /// An invalid enum variant was unpacked
    InvalidEnumVariant {
        /// The name of the enum to give hints
        name: String,
        /// The invalid variant
        variant: usize,
    },
    /// An IO error happened during unpacking
    IO(std::io::Error),
}

impl From<std::io::Error> for UnpackError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

/// Allows the implementing structs to be serialized and
/// deserialized from a binary stream
pub trait Packable: Sized {
    /// Pack `self` to `w`
    /// # Arguments
    /// * `w` - The stream to write to using `.write_all()`
    /// * `big_endian` - Whether the stream should be written to in big endian form
    fn pack<W: io::Write>(self, w: &mut W, big_endian: bool) -> Result<(), io::Error>;

    /// Unpack `Self` from `r`
    /// # Arguments
    /// * `r` - The stream to read from
    /// * `big_endian` - Whether the stream should be read from in big endian form
    fn unpack<R: io::Read>(r: &mut R, big_endian: bool) -> Result<Self, UnpackError>;
}

macro_rules! impl_packable {
    ($i:ident) => {
        impl Packable for $i {
            fn pack<W: io::Write>(self, w: &mut W, big_endian: bool) -> Result<(), io::Error> {
                if big_endian {
                    w.write_all(&self.to_be_bytes())
                } else {
                    w.write_all(&self.to_le_bytes())
                }
            }

            fn unpack<R: io::Read>(r: &mut R, big_endian: bool) -> Result<Self, UnpackError> {
                let mut data = [0u8; core::mem::size_of::<Self>()];

                r.read_exact(&mut data)?;

                if big_endian {
                    Ok(Self::from_be_bytes(data))
                } else {
                    Ok(Self::from_le_bytes(data))
                }
            }
        }
    };
}

impl_packable!(u8);
impl_packable!(i8);
impl_packable!(u16);
impl_packable!(i16);
impl_packable!(u32);
impl_packable!(i32);
impl_packable!(u64);
impl_packable!(i64);
impl_packable!(u128);
impl_packable!(i128);