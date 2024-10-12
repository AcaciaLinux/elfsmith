use std::{fmt::Debug, io};

/// Just a binary blob
pub struct Blob {
    /// The contained data
    pub blob: Vec<u8>,
}

impl Debug for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blob of {} bytes", self.blob.len())
    }
}

impl Blob {
    /// Loads a blob from `r`
    /// # Arguments
    /// * `r` - The stream to read from
    /// * `offset` - The offset where to read from the stream
    /// * `size` - The amount of bytes to read from the stream
    pub fn load<R: io::Read + io::Seek>(
        r: &mut R,
        offset: u64,
        size: usize,
    ) -> Result<Self, io::Error> {
        let mut res = vec![0u8; size];

        let old_pos = r.stream_position()?;
        r.seek(io::SeekFrom::Start(offset))?;
        r.read_exact(&mut res)?;
        r.seek(io::SeekFrom::Start(old_pos))?;

        Ok(Blob { blob: res })
    }
}
