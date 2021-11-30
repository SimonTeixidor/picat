use std::fmt;

pub enum Error {
    Image(image::ImageError),
    Arg(lexopt::Error),
    Io {
        context: String,
        error: std::io::Error,
    },
    Liq(imagequant::liq_error),
}

impl From<imagequant::liq_error> for Error {
    fn from(e: imagequant::liq_error) -> Self {
        Error::Liq(e)
    }
}

impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Self {
        Error::Image(e)
    }
}

impl From<lexopt::Error> for Error {
    fn from(e: lexopt::Error) -> Self {
        Error::Arg(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io { context, error } => {
                write!(f, "I/O error: when {}: {}", context, error)
            }
            Error::Image(e) => {
                write!(f, "Error when processing image: {}", e)
            }
            Error::Liq(e) => {
                write!(f, "Error when processing image: {}", e)
            }
            Error::Arg(e) => {
                write!(f, "Error when parsing command line arguments: {}", e)
            }
        }
    }
}
