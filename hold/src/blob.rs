use std::fmt::{self, Debug, Formatter};
use std::io;

use bytes::Bytes;
use futures::{stream, Stream};
use std::pin::Pin;

type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync + 'static>>;

/// A blob is an object that can be stored onto a provider
pub struct Blob {
    /// A blob key is a generic unique identifier for the blob.
    /// It roughly maps to a file path in a traditional filesystem.
    key: String,

    /// Total binary size in bytes of the blob.
    size: usize,

    /// The actual binary content of the blob.
    content_stream: ByteStream,
}

impl Blob {
    pub fn new<K: ToString, S: Stream<Item = Result<Bytes, io::Error>> + Send + Sync + 'static>(
        key: K,
        size: usize,
        stream: S,
    ) -> Self {
        Self {
            key: key.to_string(),
            size,
            content_stream: Box::pin(stream),
        }
    }

    pub fn from_bytes<K: ToString>(key: K, content: Vec<u8>) -> Self {
        Self::new(
            key,
            content.len(),
            stream::once(async move { Ok(Bytes::from(content)) }),
        )
    }
    
    pub fn empty<K: ToString>(key: K, size: usize) -> Self {
        Self::new(key, size, stream::empty())
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn into_byte_stream(self) -> impl Stream<Item = Result<Bytes, io::Error>> {
        self.content_stream
    }
}

impl Debug for Blob {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blob")
            .field("key", &self.key)
            .field("size", &self.size)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::blob::Blob;

    #[test]
    fn it_builds_a_new_blob() {
        let bytes = rand::thread_rng().gen::<[u8; 32]>().to_vec();
        let blob = Blob::from_bytes(String::from("key"), bytes.clone());

        assert_eq!(blob.key(), "key");
        assert_eq!(blob.size(), bytes.len());
    }
}
