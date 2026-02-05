#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "出来高と売買代金を表現する"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VolumeInfo\","]
#[doc = "  \"description\": \"出来高と売買代金を表現する\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"symbol\","]
#[doc = "    \"timestamp\","]
#[doc = "    \"volume\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"open_interest\": {"]
#[doc = "      \"description\": \"建玉残（未決済契約数）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"quote_volume\": {"]
#[doc = "      \"description\": \"売買代金（決済通貨建て）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"数量\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"symbol\": {"]
#[doc = "      \"description\": \"銘柄識別子\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"ISO 8601形式のタイムスタンプ (UTC)\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"volume\": {"]
#[doc = "      \"description\": \"数量\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VolumeInfo {
    #[doc = "建玉残（未決済契約数）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub open_interest: ::std::option::Option<f64>,
    #[doc = "売買代金（決済通貨建て）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quote_volume: ::std::option::Option<f64>,
    #[doc = "銘柄識別子"]
    pub symbol: VolumeInfoSymbol,
    #[doc = "ISO 8601形式のタイムスタンプ (UTC)"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    pub volume: f64,
}
impl ::std::convert::From<&VolumeInfo> for VolumeInfo {
    fn from(value: &VolumeInfo) -> Self {
        value.clone()
    }
}
impl VolumeInfo {
    pub fn builder() -> builder::VolumeInfo {
        Default::default()
    }
}
#[doc = "銘柄識別子"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"銘柄識別子\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VolumeInfoSymbol(::std::string::String);
impl ::std::ops::Deref for VolumeInfoSymbol {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VolumeInfoSymbol> for ::std::string::String {
    fn from(value: VolumeInfoSymbol) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VolumeInfoSymbol> for VolumeInfoSymbol {
    fn from(value: &VolumeInfoSymbol) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for VolumeInfoSymbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VolumeInfoSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VolumeInfoSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VolumeInfoSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VolumeInfoSymbol {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct VolumeInfo {
        open_interest: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        quote_volume: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        symbol: ::std::result::Result<super::VolumeInfoSymbol, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        volume: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for VolumeInfo {
        fn default() -> Self {
            Self {
                open_interest: Ok(Default::default()),
                quote_volume: Ok(Default::default()),
                symbol: Err("no value supplied for symbol".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                volume: Err("no value supplied for volume".to_string()),
            }
        }
    }
    impl VolumeInfo {
        pub fn open_interest<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_interest = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_interest: {}", e));
            self
        }
        pub fn quote_volume<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.quote_volume = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for quote_volume: {}", e));
            self
        }
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VolumeInfoSymbol>,
            T::Error: ::std::fmt::Display,
        {
            self.symbol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbol: {}", e));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
            self
        }
        pub fn volume<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.volume = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for volume: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<VolumeInfo> for super::VolumeInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VolumeInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                open_interest: value.open_interest?,
                quote_volume: value.quote_volume?,
                symbol: value.symbol?,
                timestamp: value.timestamp?,
                volume: value.volume?,
            })
        }
    }
    impl ::std::convert::From<super::VolumeInfo> for VolumeInfo {
        fn from(value: super::VolumeInfo) -> Self {
            Self {
                open_interest: Ok(value.open_interest),
                quote_volume: Ok(value.quote_volume),
                symbol: Ok(value.symbol),
                timestamp: Ok(value.timestamp),
                volume: Ok(value.volume),
            }
        }
    }
}
