use serde::{Deserialize, Serialize};
use std::ops::Deref;
use thiserror::Error;
use url::{ParseError, Url};

/// Wrapper around Box<str> which limits it's length to the constant parameter of L, used to enforce PluralKit's length
/// limits within the library, while not necessarily, this avoids sending any requests which will obviously fail.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "&str")]
pub struct LimitedStr<const L: usize>(Box<str>);

impl<const L: usize> LimitedStr<L> {
	/// # Safety
	/// While not unsafe in the memory handling sense, this function will allow you to bypass the length checks, if the
	/// string exceeds the limit, then using it in any API requests will result in an error.
	pub unsafe fn new_unchecked<S: Into<Box<str>>>(str: S) -> Self {
		LimitedStr(str.into())
	}
}

impl<const L: usize> Deref for LimitedStr<L> {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<const L: usize> ToString for LimitedStr<L> {
	fn to_string(&self) -> String {
		self.0.to_string()
	}
}

impl<'a, const L: usize> TryFrom<&'a str> for LimitedStr<L> {
	type Error = ExceededLimitError<'a>;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		match value.len() > L {
			true => Err(ExceededLimitError(value, L)),
			false => Ok(Self(value.into())),
		}
	}
}

#[derive(Error, Debug, Eq, PartialEq)]
#[error("&str \"{0}\" should not exceed length {1}")]
pub struct ExceededLimitError<'a>(&'a str, usize);

/// Wrapper around Url which limits it's length to the constant parameter of L, used to enforce PluralKit's length
/// limits within the library, while not necessarily, this avoids sending any requests which will obviously fail.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "&str")]
pub struct LimitedUrl<const L: usize>(Url);

impl<const L: usize> LimitedUrl<L> {
	/// # Safety
	/// While not unsafe in the memory handling sense, this function will allow you to bypass length checks, if the url
	/// exceeds the limit, then using it in any API requests will result in an error.
	pub const unsafe fn new_unchecked(url: Url) -> Self {
		LimitedUrl(url)
	}
}

impl<const L: usize> Deref for LimitedUrl<L> {
	type Target = Url;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<const L: usize> From<Url> for LimitedUrl<L> {
	fn from(value: Url) -> Self {
		Self(value)
	}
}

impl<'a, const L: usize> TryFrom<&'a str> for LimitedUrl<L> {
	type Error = LimitedUrlError<'a>;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		match value.len() > L {
			true => Err(LimitedUrlError::ExceededLimitError(value, L)),
			false => Ok(Self(Url::parse(value)?)),
		}
	}
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum LimitedUrlError<'a> {
	#[error("Url \"{0}\" should not exceed length {1}")]
	ExceededLimitError(&'a str, usize),
	#[error(transparent)]
	ParseError(#[from] ParseError),
}
