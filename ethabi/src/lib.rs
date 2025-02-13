// Copyright 2015-2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Ethereum ABI encoding decoding library.

#![allow(clippy::module_inception)]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

mod constructor;
mod contract;
mod decoder;
mod encoder;
mod errors;
mod event;
mod event_param;
mod filter;
mod function;
mod log;
mod operation;
mod param;
pub mod param_type;
mod signature;
pub mod token;
mod tuple_param;
mod util;

#[cfg(test)]
mod tests;

pub use crate::{
	constructor::Constructor,
	contract::{Contract, Events, Functions},
	decoder::decode,
	encoder::encode,
	encoder::encode_packed,
	errors::{Error, Result},
	event::Event,
	event_param::EventParam,
	filter::{RawTopicFilter, Topic, TopicFilter},
	function::Function,
	log::{Log, LogFilter, LogParam, ParseLog, RawLog},
	param::Param,
	param_type::ParamType,
	token::Token,
	tuple_param::TupleParam,
};

/// ABI word.
pub type Word = [u8; 32];

/// ABI address.
pub type Address = ethereum_types::Address;

/// ABI fixed bytes.
pub type FixedBytes = Vec<u8>;

/// ABI bytes.
pub type Bytes = Vec<u8>;

/// ABI signed integer.
pub type Int = ethereum_types::U256;

/// ABI unsigned integer.
pub type Uint = ethereum_types::U256;

/// Commonly used FixedBytes of size 32
pub type Hash = ethereum_types::H256;

/// Contract functions generated by ethabi-derive
pub trait FunctionOutputDecoder {
	/// Output types of the contract function
	type Output;

	/// Decodes the given bytes output for the contract function
	fn decode(&self, _: &[u8]) -> Result<Self::Output>;
}
