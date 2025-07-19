use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod reexports;

pub trait Id<'de>: From<Uuid> + Serialize + Deserialize<'de> + std::fmt::Display {
    const PREFIX: &'static str;

    fn get(self) -> Uuid;
}

pub const UUID_STRING_LENGTH: usize = 36;
pub const SEPARATOR: char = '-';
pub const SEPARATOR_LENGTH: usize = SEPARATOR.len_utf8();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdParseError {
    InvalidPrefix,
    InvalidUuid(uuid::Error),
}

impl std::fmt::Display for IdParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdParseError::InvalidPrefix => write!(f, "failed to parse ID: invalid prefix"),
            IdParseError::InvalidUuid(e) => write!(f, "failed to parse ID: invalid UUID: {e}"),
        }
    }
}

impl std::error::Error for IdParseError {}

#[macro_export]
macro_rules! define_id {
    ($name:ident, $prefix:expr) => {
        define_id!($name, $prefix, pref_id);
    };
    ($name:ident, $prefix:expr, $id_crate:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(uuid::Uuid);

        impl<'de> $id_crate::Id<'de> for $name {
            const PREFIX: &'static str = $prefix;

            fn get(self) -> uuid::Uuid {
                self.0
            }
        }

        impl From<$id_crate::reexports::uuid::Uuid> for $name {
            fn from(id: $id_crate::reexports::uuid::Uuid) -> Self {
                Self(id)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $id_crate::IdParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use $id_crate::Id;

                if !s.starts_with(Self::PREFIX)
                    || s.as_bytes().get(Self::PREFIX.len()) != Some(&b'-')
                {
                    return Err($id_crate::IdParseError::InvalidPrefix);
                }

                let uuid = $id_crate::reexports::uuid::Uuid::parse_str(&s[Self::PREFIX.len() + 1..])
                    .map_err(|e| $id_crate::IdParseError::InvalidUuid(e))?;

                Ok(Self(uuid.into()))
            }
        }

        impl $id_crate::reexports::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $id_crate::reexports::serde::Serializer,
            {
                use ::std::fmt::Write;

                use $id_crate::Id;

                let mut buf = String::with_capacity(
                    Self::PREFIX.len()
                        + $id_crate::UUID_STRING_LENGTH
                        + $id_crate::SEPARATOR_LENGTH,
                );
                buf.push_str(Self::PREFIX);
                buf.push($id_crate::SEPARATOR);
                write!(buf, "{}", self.0).unwrap();
                serializer.serialize_str(&buf)
            }
        }

        impl<'de> $id_crate::reexports::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $id_crate::reexports::serde::Deserializer<'de>,
            {
                use ::std::str::FromStr;

                let s = String::deserialize(deserializer)?;

                Self::from_str(&s).map_err(|e| serde::de::Error::custom(e))
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use $id_crate::Id;

                write!(f, "{}-{}", Self::PREFIX, self.0)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    define_id!(TestId, "test", crate);

    #[test]
    fn test_id() {
        use serde_test::{Token, assert_tokens};

        let id = uuid::Uuid::nil();
        let test_id = TestId::from(id);

        const EXPECTED_STRING: &str = "test-00000000-0000-0000-0000-000000000000";
        assert_eq!(
            EXPECTED_STRING.len(),
            UUID_STRING_LENGTH + SEPARATOR_LENGTH + TestId::PREFIX.len()
        );

        assert_tokens(&test_id, &[Token::String(EXPECTED_STRING)]);
    }

    #[test]
    fn test_id_parse() {
        assert_eq!(
            TestId::from_str("test-00000000-0000-0000-0000-000000000000")
                .expect("this prefixed id should be valid"),
            TestId::from(uuid::Uuid::nil())
        );

        assert_eq!(
            TestId::from_str("test1-00000000-0000-0000-0000-000000000000")
                .expect_err("incorrect prefix should be invalid"),
            IdParseError::InvalidPrefix
        );

        assert!(matches!(
            TestId::from_str("test-00000000-0000-0000-0000-00")
                .expect_err("incorrect UUID should be invalid"),
            IdParseError::InvalidUuid(_)
        ));
    }
}
