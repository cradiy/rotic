#[derive(Debug)]
pub enum Error {
    InvalidFormat,
    #[allow(clippy::enum_variant_names)]
    IoError(std::io::Error),
    Custom(String),
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidFormat => f.write_str("Invalid format"),
            Error::Custom(err) => f.write_str(err),
            Error::IoError(err) => std::fmt::Display::fmt(err, f),
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}