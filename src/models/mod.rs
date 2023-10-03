pub mod member;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Privacy {
	Public,
	Private,
}

// I have reinvented Option<T>
#[derive(Clone, Debug, Default, Serialize)]
pub enum Patchable<T: Clone + Debug + Serialize> {
	Patched(T),
	#[default]
	Unmodified,
}

impl<T: Clone + Debug + Serialize> Patchable<T> {
	fn is_unmodified(&self) -> bool {
		match self {
			Patchable::Patched(_) => false,
			Patchable::Unmodified => true,
		}
	}
}

mod color {
	use rgb::RGB8;
	use serde::{de, Deserialize, Deserializer, Serializer};

	pub fn serialize<S: Serializer>(
		color: &Option<RGB8>,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		match color {
			None => serializer.serialize_none(),
			Some(color) => serializer.serialize_str(&hex::encode(color)),
		}
	}

	pub fn deserialize<'d, D: Deserializer<'d>>(deserializer: D) -> Result<Option<RGB8>, D::Error> {
		let hex = match Option::<&str>::deserialize(deserializer)? {
			None => return Ok(None),
			Some(value) => value,
		};

		let values = hex::decode(hex).map_err(de::Error::custom)?;

		Ok(Some(RGB8::new(values[0], values[1], values[2])))
	}
}

mod patchable_color {
	use crate::models::{color, Patchable};
	use rgb::RGB8;
	use serde::{ser::Error, Serializer};

	pub fn serialize<S: Serializer>(
		color: &Patchable<Option<RGB8>>,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		match color {
			Patchable::Patched(color) => color::serialize(color, serializer),
			Patchable::Unmodified => Err(Error::custom(
				"unmodified patchable should not be serialized",
			)),
		}
	}
}

mod patchable_datetime {
	use crate::models::Patchable;
	use serde::{ser::Error, Serializer};
	use time::{serde::iso8601, OffsetDateTime};

	pub fn serialize<S: Serializer>(
		datetime: &Patchable<Option<OffsetDateTime>>,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		match datetime {
			Patchable::Patched(datetime) => iso8601::option::serialize(datetime, serializer),
			Patchable::Unmodified => Err(Error::custom(
				"unmodified patchable should not be serialized",
			)),
		}
	}
}
