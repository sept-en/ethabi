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

#[cfg(feature = "std")]
error_chain! {
	foreign_links {
		SerdeJson(serde_json::Error);
		ParseInt(num::ParseIntError);
		Utf8(string::FromUtf8Error);
		Hex(hex::FromHexError);
	}

	errors {
		InvalidName(name: String) {
			description("Invalid name"),
			display("Invalid name `{}`", name),
		}

		InvalidData {
			description("Invalid data"),
			display("Invalid data"),
		}
	}
}

#[cfg(feature = "no_std")]
error_chain! {
	foreign_links {
		ParseInt(num::ParseIntError);
		Utf8(string::FromUtf8Error);
		Hex(hex::FromHexError);
	}

	errors {
		InvalidName(name: String) {
			description("Invalid name"),
			display("Invalid name `{}`", name),
		}

		InvalidData {
			description("Invalid data"),
			display("Invalid data"),
		}
	}
}
