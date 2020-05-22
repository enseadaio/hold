use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("ID not found {}: {}", id, source))]
    IDNotFound {
        id: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[snafu(display("Provider error: {}", source))]
    ProviderError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl Error {
    pub fn not_found<E: 'static + std::error::Error + Send + Sync>(id: String, source: E) -> Self {
        Error::IDNotFound {
            id,
            source: Box::new(source),
        }
    }

    pub fn provider<E: 'static + std::error::Error + Send + Sync>(source: E) -> Self {
        Error::ProviderError {
            source: Box::new(source),
        }
    }
}

impl From<Error> for std::io::Error {
    fn from(err: Error) -> Self {
        use std::io::ErrorKind;
        match err {
            Error::IDNotFound { source, .. } => Self::new(ErrorKind::NotFound, source),
            Error::ProviderError { source, .. } => Self::new(ErrorKind::Other, source),
        }
    }
}
