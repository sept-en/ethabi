#![allow(unknown_lints)]
#![allow(missing_docs)]

use rstd::num;

#[cfg(feature = "std")]
use std::string;

#[cfg(feature = "no_std")]
use alloc::string;

#[cfg(feature = "std")]
use serde_json;

use hex;

#[derive(Debug)]
pub enum ErrorKind {
    InvalidName(String),
    InvalidData,
}

impl From<String> for ErrorKind {
    fn from(s: String) -> Self {
        ErrorKind::InvalidName(s)
    }
}

impl From<&str> for ErrorKind {
    fn from(s: &str) -> Self {
        ErrorKind::InvalidName(s.parse().unwrap())
    }
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::InvalidName(_) => "Invalid name",
            ErrorKind::InvalidData => "Invalid data",
        }
    }
}

impl rstd::fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut rstd::fmt::Formatter) -> rstd::fmt::Result {
        match *self {
            ErrorKind::InvalidName(ref e) => write!(fmt, "{}", e),
            ErrorKind::InvalidData => write!(fmt, "Invalid data"),
        }
    }
}



#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "std")]
    SerdeJson(serde_json::Error),
    ParseInt(num::ParseIntError),
    Utf8(string::FromUtf8Error),
    Hex(hex::FromHexError),
    ErrorKind(ErrorKind),
}

//#[cfg(feature = "no_std")]
//#[derive(Debug)]
//pub enum Error {
//    ParseInt(num::ParseIntError),
//    Utf8(string::FromUtf8Error),
//    Hex(hex::FromHexError),
//    ErrorKind(ErrorKind),
//}

pub type Result<T> = rstd::result::Result<T, Error>;

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::ErrorKind(s.into())
    }
}

#[cfg(feature = "std")]
impl rstd::fmt::Display for Error {
    fn fmt(&self, fmt: &mut rstd::fmt::Formatter) -> rstd::fmt::Result {
        match *self {
            Error::SerdeJson(ref e) => write!(fmt, "{}", e),
            Error::ParseInt(ref e) => write!(fmt, "{}", e),
            Error::Utf8(ref e) => write!(fmt, "{}", e),
            Error::Hex(ref e) => write!(fmt, "{}", e),
            Error::ErrorKind(ref e) => write!(fmt, "{}", e),
        }
    }
}

#[cfg(feature = "no_std")]
impl rstd::fmt::Display for Error {
    fn fmt(&self, fmt: &mut rstd::fmt::Formatter) -> rstd::fmt::Result {
        match *self {
            Error::SerdeJson(ref e) => write!(fmt, "{}", e),
            Error::ParseInt(ref e) => write!(fmt, "{}", e),
            Error::Utf8(ref e) => write!(fmt, "{}", e),
            Error::Hex(ref e) => write!(fmt, "{}", e),
            Error::ErrorKind(ref e) => write!(fmt, "{}", e),
        }
    }
}

//impl rstd::error::Error for Error {
//    fn description(&self) -> &str { "Fetch client error" }
//    fn cause(&self) -> Option<&dyn std::error::Error> { None }
//}

#[cfg(feature = "std")]
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self {
        Error::Utf8(e)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::Hex(e)
    }
}

impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Error {
        Error::ErrorKind(kind)
    }
}

pub trait ResultExt<T> {
    /// If the `Result` is an `Err` then `chain_err` evaluates the closure,
    /// which returns *some type that can be converted to `ErrorKind`*, boxes
    /// the original error to store as the cause, then returns a new error
    /// containing the original error.
    fn chain_err<F, EK>(self, callback: F) -> rstd::result::Result<T, Error> where F: FnOnce() -> EK, EK: Into<ErrorKind>;
}

//impl<T, E> ResultExt<T> for rstd::result::Result<T, E> where E: Send + 'static {
//    fn chain_err<F, EK>(self, callback: F) -> rstd::result::Result<T, Error>
//    where F: FnOnce() -> EK,
//    EK: Into<ErrorKind> {
//        self.map_err(move |e| {
//            e
////            let state = $crate::State::new::<Error>(Box::new(e), );
////            $crate::ChainedError::new(callback().into(), state)
//        })
//    }
//}

impl<T> ResultExt<T> for rstd::result::Result<T, Error> {
    fn chain_err<F, EK>(self, callback: F) -> rstd::result::Result<T, Error>
        where F: FnOnce() -> EK,
              EK: Into<ErrorKind> {
        self.map_err(move |e| {
            e
//            let state = $crate::State::new::<Error>(Box::new(e), );
//            $crate::ChainedError::new(callback().into(), state)
        })
    }
}

impl<T> ResultExt<T> for Option<T> {
    fn chain_err<F, EK>(self, callback: F) -> rstd::result::Result<T, Error>
        where F: FnOnce() -> EK,
              EK: Into<ErrorKind> {
        self.ok_or_else(move || {
            callback().into().into()
//            $crate::ChainedError::from_kind()
        })
    }
}

//#[cfg(feature = "std")]
//error_chain! {
//	foreign_links {
//		SerdeJson(serde_json::Error);
//		ParseInt(num::ParseIntError);
//		Utf8(string::FromUtf8Error);
//		Hex(hex::FromHexError);
//	}
//
//	errors {
//		InvalidName(name: String) {
//			description("Invalid name"),
//			display("Invalid name `{}`", name),
//		}
//
//		InvalidData {
//			description("Invalid data"),
//			display("Invalid data"),
//		}
//	}
//}
