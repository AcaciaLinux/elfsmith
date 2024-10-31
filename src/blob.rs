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

    /// Writes the contents of this blob to `w` at `offset`
    /// # Arguments
    /// * `w` - The stream to write to
    /// * `offset` - The offset to start the data at
    pub fn write<W: io::Write + io::Seek>(&self, w: &mut W, offset: u64) -> Result<(), io::Error> {
        let old_pos = w.stream_position()?;
        w.seek(io::SeekFrom::Start(offset))?;
        w.write_all(&self.blob)?;
        w.seek(io::SeekFrom::Start(old_pos))?;

        Ok(())
    }
}
