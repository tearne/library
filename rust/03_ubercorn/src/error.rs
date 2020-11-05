use std::fmt;
use std::error;
use std::ffi;
use std::io;

use std::sync::mpsc;
use nix;
use libc;

#[derive(Debug)]
pub enum Error {
	/// System errors.
	Nix(nix::Error),

	/// Errors with internal nulls in names.
	Nul(ffi::NulError),

	Io(io::Error),

	Send(mpsc::SendError<libc::input_event>),

	/// The uinput file could not be found.
	NotFound,

	/// error reading input_event
	ShortRead,

	/// try recieve
	TryReceive(mpsc::TryRecvError),
	TryRecieveTimeout(mpsc::RecvTimeoutError),

	/// String parsing
	Internal(String),
}

impl From<ffi::NulError> for Error {
	fn from(value: ffi::NulError) -> Self {
		Error::Nul(value)
	}
}

impl From<nix::Error> for Error {
	fn from(value: nix::Error) -> Self {
		Error::Nix(value)
	}
}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Error::Io(value)
	}
}

impl From<mpsc::SendError<libc::input_event>> for Error {
	fn from(value: mpsc::SendError<libc::input_event>) -> Self {
		Error::Send(value)
	}
}

impl From<mpsc::TryRecvError> for Error {
	fn from(value: mpsc::TryRecvError) -> Self {
		Error::TryReceive(value)
	}
}

impl From<mpsc::RecvTimeoutError> for Error {
	fn from(value: mpsc::RecvTimeoutError) -> Self {
		Error::TryRecieveTimeout(value)
	}
}

impl From<std::string::FromUtf8Error> for Error {
	fn from(value: std::string::FromUtf8Error) -> Self {
		Error::Internal(format!("{:?}",value))
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			&Error::Nix(ref err) => err.fmt(f),

			&Error::Nul(ref err) => err.fmt(f),

			&Error::Io(ref err) => err.fmt(f),

			&Error::Send(ref err) => err.fmt(f),

			&Error::NotFound => f.write_str("Device not found."),

			&Error::ShortRead => f.write_str("Error while reading from device file."),

			&Error::TryReceive(ref err) => err.fmt(f),
			&Error::TryRecieveTimeout(ref err) => err.fmt(f),
			
			&Error::Internal(ref msg) => f.write_str(msg),
		}
	}
}

impl error::Error for Error {}
