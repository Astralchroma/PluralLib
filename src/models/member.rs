use crate::limited::{LimitedStr, LimitedUrl};
use crate::models::{Patchable, Privacy};
use crate::references::ShortId;
use rgb::RGB8;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct Member {
	pub id: ShortId,
	pub uuid: Uuid,
	#[serde(rename = "system")]
	pub system_id: ShortId,
	pub name: LimitedStr<100>,
	pub display_name: Option<LimitedStr<100>>,
	#[serde(with = "crate::models::color")]
	pub color: Option<RGB8>,
	#[serde(with = "time::serde::iso8601::option")]
	pub birthday: Option<OffsetDateTime>,
	pub pronouns: Option<LimitedStr<100>>,
	#[serde(rename = "avatar_url")]
	pub avatar: Option<LimitedUrl<256>>,
	#[serde(rename = "webhook_avatar_url")]
	pub webhook_avatar: Option<LimitedUrl<256>>,
	pub banner: Option<LimitedUrl<256>>,
	pub description: Option<LimitedStr<1000>>,
	#[serde(with = "time::serde::iso8601::option")]
	pub created: Option<OffsetDateTime>,
	pub proxy_tags: Vec<ProxyTag>,
	#[serde(rename = "keep_proxy")]
	pub keep_proxy_tags: bool,
	pub text_to_speech: bool,
	pub autoproxy_enabled: Option<bool>,
	pub message_count: Option<u32>,
	#[serde(with = "time::serde::iso8601::option")]
	pub last_message_timestamp: Option<OffsetDateTime>,
	pub privacy: Option<MemberPrivacy>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct MemberPrivacy {
	pub visibility: Privacy,
	pub name: Privacy,
	pub description: Privacy,
	pub birthday: Privacy,
	pub pronouns: Privacy,
	pub avatar: Privacy,
	pub metadata: Privacy,
}

const PROXY_TAG_SIZE_LIMIT: usize = 100;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProxyTag {
	pub prefix: Option<Box<str>>,
	pub suffix: Option<Box<str>>,
}

impl ProxyTag {
	pub fn new<S>(
		prefix: Option<S>,
		suffix: Option<S>,
	) -> Result<ProxyTag, ProxyTagExceededLimitError>
	where
		S: Into<Box<str>> + Clone + Debug + Serialize,
	{
		let mut length = 0;

		let mut validate = |parameter: Option<S>| {
			Ok(match parameter {
				Some(parameter) => {
					let string = parameter.into();
					length += string.len();
					if length > PROXY_TAG_SIZE_LIMIT {
						return Err(ProxyTagExceededLimitError);
					}
					Some(string)
				}
				None => None,
			})
		};

		Ok(ProxyTag {
			prefix: validate(prefix)?,
			suffix: validate(suffix)?,
		})
	}
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("proxy tags must not exceed 100 total characters")]
pub struct ProxyTagExceededLimitError;

#[derive(Clone, Debug, Default, Serialize)]
pub struct MemberPatch {
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub name: Patchable<LimitedStr<100>>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub display_name: Patchable<Option<LimitedStr<100>>>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	#[serde(with = "crate::models::patchable_color")]
	pub color: Patchable<Option<RGB8>>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	#[serde(with = "crate::models::patchable_datetime")]
	pub birthday: Patchable<Option<OffsetDateTime>>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub pronouns: Patchable<Option<LimitedStr<100>>>,
	#[serde(rename = "avatar_url")]
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub avatar: Patchable<Option<LimitedUrl<256>>>,
	#[serde(rename = "webhook_avatar_url")]
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub webhook_avatar: Patchable<Option<LimitedUrl<256>>>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub banner: Patchable<Option<LimitedUrl<256>>>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub description: Patchable<Option<LimitedStr<1000>>>,
	pub proxy_tags: Vec<ProxyTag>,
	#[serde(rename = "keep_proxy")]
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub keep_proxy_tags: Patchable<bool>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub text_to_speech: Patchable<bool>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub autoproxy_enabled: Patchable<Option<bool>>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub privacy: Patchable<MemberPrivacyPatch>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct MemberPrivacyPatch {
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub visibility: Patchable<Privacy>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub name: Patchable<Privacy>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub description: Patchable<Privacy>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub birthday: Patchable<Privacy>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub pronouns: Patchable<Privacy>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub avatar: Patchable<Privacy>,
	#[serde(skip_serializing_if = "Patchable::is_unmodified")]
	pub metadata: Patchable<Privacy>,
}

impl MemberPrivacyPatch {
	pub const PUBLIC: MemberPrivacyPatch = Self::all(Privacy::Public);
	pub const PRIVATE: MemberPrivacyPatch = Self::all(Privacy::Private);

	const fn all(privacy: Privacy) -> MemberPrivacyPatch {
		MemberPrivacyPatch {
			visibility: Patchable::Patched(privacy),
			name: Patchable::Patched(privacy),
			description: Patchable::Patched(privacy),
			birthday: Patchable::Patched(privacy),
			pronouns: Patchable::Patched(privacy),
			avatar: Patchable::Patched(privacy),
			metadata: Patchable::Patched(privacy),
		}
	}
}
