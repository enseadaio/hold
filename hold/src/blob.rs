/// A blob is an object that can be stored onto a provider
#[derive(Clone, Debug)]
pub struct Blob {
    /// A blob key is a generic unique identifier for the blob.
    /// It roughly maps to a file path in a traditional filesystem.
    key: String,

    /// Total binary size in bytes of the blob.
    size: usize,

    /// The actual binary content of the blob.
    content: Vec<u8>,
}

impl Blob {
    pub fn new(key: String, content: Vec<u8>) -> Blob {
        Blob {
            key,
            size: content.len().clone(),
            content,
        }
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn content(&self) -> &Vec<u8> {
        &self.content
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::blob::Blob;

    #[test]
    fn it_builds_a_new_blob() {
        let bytes = rand::thread_rng().gen::<[u8; 32]>().to_vec();
        let blob = Blob::new(String::from("key"), bytes.clone());

        assert_eq!(blob.key(), "key");
        assert_eq!(blob.content(), &bytes);
        assert_eq!(blob.size(), bytes.len());
    }

    #[test]
    fn it_can_be_cloned() {
        let bytes = rand::thread_rng().gen::<[u8; 32]>().to_vec();
        let blob = Blob::new(String::from("key"), bytes.clone());
        let clone = blob.clone();

        assert_eq!(blob.key(), clone.key());
        assert_eq!(blob.content(), clone.content());
        assert_eq!(blob.size(), clone.size());
    }
}
