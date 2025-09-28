use thiserror::Error;

#[derive(Error, Debug)]
pub enum SMSSenderError {
    #[error("I/O error: {0}")]
    Io(#[from] tokio::io::Error),
    #[error("Serde Json Error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    // #[error("Error: {0}")]
    // Other(String),
}

/// A `Result` alias where the `Err` case is `SMSSenderError`.
pub type Result<T> = std::result::Result<T, SMSSenderError>;

// we must manually implement serde::Serialize
impl serde::Serialize for SMSSenderError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
