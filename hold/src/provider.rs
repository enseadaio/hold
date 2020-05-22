use async_trait::async_trait;

use crate::blob::Blob;
use crate::Result;

/// An abstract storage provider
#[async_trait]
pub trait Provider {
    /// Fetches a blob from the storage provider given its key
    async fn get_blob(&self, key: &str) -> Result<Option<Blob>>;

    /// Stores the given blob and returns it back
    async fn store_blob(&self, blob: Blob) -> Result<Blob>;

    /// Checks if the blob exists. Some implementation may still be
    /// loading the blob content in memory if the underlying implementation
    /// does not support headless lookups.
    async fn is_blob_present(&self, key: &str) -> Result<bool>;

    /// Fetches a blob from the storage provider given its key
    async fn delete_blob(&self, key: &str) -> Result<()>;
}
