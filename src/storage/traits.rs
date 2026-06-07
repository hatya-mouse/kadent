use std::io::{Cursor, Read};

pub trait AsBytes {
    /// Converts the self into bytes so it can be loaded later.
    fn as_bytes(&self, bytes: &mut Vec<u8>);
}

pub trait FromBytes: Sized {
    /// Loads the instance of self from bytes.
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self>;
}

pub fn safe_read(cursor: &mut Cursor<&[u8]>, len: usize) -> std::io::Result<Vec<u8>> {
    // Check if there are enough bytes to read
    if cursor.position() + len as u64 > cursor.get_ref().len() as u64 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Not enough bytes to read",
        ));
    }

    // Read the bytes into a buffer
    let mut bytes = vec![0u8; len];
    cursor.read_exact(&mut bytes)?;

    Ok(bytes)
}
