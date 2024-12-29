use std::fmt;
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    UnsupportedSegwitFlag(u8),
    ParseFailed(&'static str),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => write!(f, "IO error: {}", e),
            Error::UnsupportedSegwitFlag(swflag) => {
                write!(f, "unsupported segwit version: {}", swflag)
            }
            Error::ParseFailed(s) => write!(f, "parse failed: {}", s),
        }
    }
}
