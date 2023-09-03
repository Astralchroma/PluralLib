use serde::{Deserialize, Serialize};
use std::ops::Deref;
use thiserror::Error;
use uuid::Uuid;

/// This represents a PluralKit Short Id. These consist of 5 a-z characters (Example: "ptckn"). These are used for
/// systems, groups, and members. These ids are ideal for any user facing interaction, however as they can be changed
/// by PluralKit's team upon request, they should not be considered a reliable unique id for the purposes of data
/// storage, please use the Uuid for that purpose.
///
/// See PluralKit Documentation: <https://pluralkit.me/api/models/#notes-on-ids>
///
/// # Future changes
/// This format is expected to change to 6 characters, with an optional `-` in the middle, with old ids remaining valid.
/// When this happens, the library will be updated, while this should not be a breaking change, however you have been
/// warned.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "&str")]
pub struct ShortId(Box<str>);

impl Deref for ShortId {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ToString for ShortId {
	fn to_string(&self) -> String {
		self.0.to_string()
	}
}

impl<'a> TryFrom<&'a str> for ShortId {
	type Error = ShortError;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		if value.len() != 5 {
			return Err(ShortError::IncorrectLength);
		}

		for char in value.chars() {
			if !char.is_ascii_lowercase() {
				return Err(ShortError::InvalidCharacters);
			}
		}

		Ok(ShortId(value.into()))
	}
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ShortError {
	#[error("A ShortId should only contain alphabetical characters (a-z)")]
	InvalidCharacters,
	#[error("A ShortId should only be 5 characters in length")]
	IncorrectLength,
}

/// This represents a reference to a Member or Group. This can either be a `ShortId` or a `Uuid`. Note that `Ref` is not
/// used for a reference to a system due to additional reference types, so for that, see `SystemRef`.
///
/// See PluralKit Documentation: <https://pluralkit.me/api/models/#notes-on-ids>
pub enum GenericRef {
	/// Reference a member or group by it's `Short`. (example: "ptckn")
	ShortId(ShortId),
	/// Reference a member or group by it's `Uuid`. (example: "30523e4f-dd68-4b91-8ee0-59c7598db16c")
	Uuid(Uuid),
}

impl ToString for GenericRef {
	fn to_string(&self) -> String {
		match self {
			GenericRef::ShortId(short) => short.to_string(),
			GenericRef::Uuid(uuid) => uuid.to_string(),
		}
	}
}

impl<'a> TryFrom<&'a str> for GenericRef {
	type Error = ShortError;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		Ok(GenericRef::ShortId(ShortId::try_from(value)?))
	}
}

impl From<Uuid> for GenericRef {
	fn from(value: Uuid) -> Self {
		Self::Uuid(value)
	}
}

/// This represents a reference to a System. This can either be a `ShortId`, `Uuid`, `Snowflake`, or `Current`. Note
/// that `SystemRef` is not used for a reference to a group or member due to lacking reference types, so for that, see
/// `GenericRef`.
///
/// See PluralKit Documentation: <https://pluralkit.me/api/endpoints/#systems>
pub enum SystemRef {
	/// Reference a system by it's `Short`. (example: "rwqjp")
	ShortId(ShortId),
	/// Reference a system by it's `Uuid`. (example: "deb31677-c36c-41db-bef5-5d1e8e2f3ad7")
	Uuid(Uuid),
	/// Reference a system by it's Discord account id. (example: 521031433972744193)
	Snowflake(u64),
	/// Reference to the currently authenticated system.
	Current,
}

impl ToString for SystemRef {
	fn to_string(&self) -> String {
		match self {
			SystemRef::ShortId(short) => short.to_string(),
			SystemRef::Uuid(uuid) => uuid.to_string(),
			SystemRef::Snowflake(snowflake) => snowflake.to_string(),
			SystemRef::Current => String::from("@me"),
		}
	}
}

impl<'a> TryFrom<&'a str> for SystemRef {
	type Error = ShortError;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		Ok(SystemRef::ShortId(ShortId::try_from(value)?))
	}
}

impl From<Uuid> for SystemRef {
	fn from(value: Uuid) -> Self {
		Self::Uuid(value)
	}
}

impl From<u64> for SystemRef {
	fn from(value: u64) -> Self {
		Self::Snowflake(value)
	}
}
