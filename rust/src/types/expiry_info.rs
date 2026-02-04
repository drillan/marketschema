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
#[doc = "先物・オプションの満期関連情報を表現する"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ExpiryInfo\","]
#[doc = "  \"description\": \"先物・オプションの満期関連情報を表現する\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expiration_date\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expiration_date\": {"]
#[doc = "      \"description\": \"日付（YYYY-MM-DD形式）\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^\\\\d{4}-\\\\d{2}-\\\\d{2}$\""]
#[doc = "    },"]
#[doc = "    \"expiry\": {"]
#[doc = "      \"description\": \"満期系列識別子（YYYY-MM, YYYY-Www, YYYY-MM-DD形式）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"満期系列識別子（YYYY-MM, YYYY-Www, YYYY-MM-DD形式）\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^\\\\d{4}(-\\\\d{2}|-W\\\\d{2}|-\\\\d{2}-\\\\d{2})$\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"last_trading_day\": {"]
#[doc = "      \"description\": \"取引可能な最終日\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"日付（YYYY-MM-DD形式）\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^\\\\d{4}-\\\\d{2}-\\\\d{2}$\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"settlement_date\": {"]
#[doc = "      \"description\": \"決済日\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"日付（YYYY-MM-DD形式）\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^\\\\d{4}-\\\\d{2}-\\\\d{2}$\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExpiryInfo {
    #[doc = "日付（YYYY-MM-DD形式）"]
    pub expiration_date: ExpiryInfoExpirationDate,
    #[doc = "満期系列識別子（YYYY-MM, YYYY-Www, YYYY-MM-DD形式）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub expiry: ::std::option::Option<ExpiryInfoExpiry>,
    #[doc = "取引可能な最終日"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub last_trading_day: ::std::option::Option<ExpiryInfoLastTradingDay>,
    #[doc = "決済日"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub settlement_date: ::std::option::Option<ExpiryInfoSettlementDate>,
}
impl ::std::convert::From<&ExpiryInfo> for ExpiryInfo {
    fn from(value: &ExpiryInfo) -> Self {
        value.clone()
    }
}
impl ExpiryInfo {
    pub fn builder() -> builder::ExpiryInfo {
        Default::default()
    }
}
#[doc = "日付（YYYY-MM-DD形式）"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"日付（YYYY-MM-DD形式）\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^\\\\d{4}-\\\\d{2}-\\\\d{2}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExpiryInfoExpirationDate(::std::string::String);
impl ::std::ops::Deref for ExpiryInfoExpirationDate {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExpiryInfoExpirationDate> for ::std::string::String {
    fn from(value: ExpiryInfoExpirationDate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ExpiryInfoExpirationDate> for ExpiryInfoExpirationDate {
    fn from(value: &ExpiryInfoExpirationDate) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ExpiryInfoExpirationDate {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^\\d{4}-\\d{2}-\\d{2}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d{4}-\\d{2}-\\d{2}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExpiryInfoExpirationDate {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExpiryInfoExpirationDate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExpiryInfoExpirationDate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExpiryInfoExpirationDate {
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
#[doc = "満期系列識別子（YYYY-MM, YYYY-Www, YYYY-MM-DD形式）"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"満期系列識別子（YYYY-MM, YYYY-Www, YYYY-MM-DD形式）\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^\\\\d{4}(-\\\\d{2}|-W\\\\d{2}|-\\\\d{2}-\\\\d{2})$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExpiryInfoExpiry(::std::string::String);
impl ::std::ops::Deref for ExpiryInfoExpiry {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExpiryInfoExpiry> for ::std::string::String {
    fn from(value: ExpiryInfoExpiry) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ExpiryInfoExpiry> for ExpiryInfoExpiry {
    fn from(value: &ExpiryInfoExpiry) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ExpiryInfoExpiry {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^\\d{4}(-\\d{2}|-W\\d{2}|-\\d{2}-\\d{2})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^\\d{4}(-\\d{2}|-W\\d{2}|-\\d{2}-\\d{2})$\"".into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExpiryInfoExpiry {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExpiryInfoExpiry {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExpiryInfoExpiry {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExpiryInfoExpiry {
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
#[doc = "日付（YYYY-MM-DD形式）"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"日付（YYYY-MM-DD形式）\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^\\\\d{4}-\\\\d{2}-\\\\d{2}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExpiryInfoLastTradingDay(::std::string::String);
impl ::std::ops::Deref for ExpiryInfoLastTradingDay {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExpiryInfoLastTradingDay> for ::std::string::String {
    fn from(value: ExpiryInfoLastTradingDay) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ExpiryInfoLastTradingDay> for ExpiryInfoLastTradingDay {
    fn from(value: &ExpiryInfoLastTradingDay) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ExpiryInfoLastTradingDay {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^\\d{4}-\\d{2}-\\d{2}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d{4}-\\d{2}-\\d{2}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExpiryInfoLastTradingDay {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExpiryInfoLastTradingDay {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExpiryInfoLastTradingDay {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExpiryInfoLastTradingDay {
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
#[doc = "日付（YYYY-MM-DD形式）"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"日付（YYYY-MM-DD形式）\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^\\\\d{4}-\\\\d{2}-\\\\d{2}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExpiryInfoSettlementDate(::std::string::String);
impl ::std::ops::Deref for ExpiryInfoSettlementDate {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExpiryInfoSettlementDate> for ::std::string::String {
    fn from(value: ExpiryInfoSettlementDate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ExpiryInfoSettlementDate> for ExpiryInfoSettlementDate {
    fn from(value: &ExpiryInfoSettlementDate) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ExpiryInfoSettlementDate {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^\\d{4}-\\d{2}-\\d{2}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d{4}-\\d{2}-\\d{2}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExpiryInfoSettlementDate {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExpiryInfoSettlementDate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExpiryInfoSettlementDate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExpiryInfoSettlementDate {
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
    pub struct ExpiryInfo {
        expiration_date:
            ::std::result::Result<super::ExpiryInfoExpirationDate, ::std::string::String>,
        expiry: ::std::result::Result<
            ::std::option::Option<super::ExpiryInfoExpiry>,
            ::std::string::String,
        >,
        last_trading_day: ::std::result::Result<
            ::std::option::Option<super::ExpiryInfoLastTradingDay>,
            ::std::string::String,
        >,
        settlement_date: ::std::result::Result<
            ::std::option::Option<super::ExpiryInfoSettlementDate>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ExpiryInfo {
        fn default() -> Self {
            Self {
                expiration_date: Err("no value supplied for expiration_date".to_string()),
                expiry: Ok(Default::default()),
                last_trading_day: Ok(Default::default()),
                settlement_date: Ok(Default::default()),
            }
        }
    }
    impl ExpiryInfo {
        pub fn expiration_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExpiryInfoExpirationDate>,
            T::Error: ::std::fmt::Display,
        {
            self.expiration_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expiration_date: {}", e));
            self
        }
        pub fn expiry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExpiryInfoExpiry>>,
            T::Error: ::std::fmt::Display,
        {
            self.expiry = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expiry: {}", e));
            self
        }
        pub fn last_trading_day<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExpiryInfoLastTradingDay>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_trading_day = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for last_trading_day: {}",
                    e
                )
            });
            self
        }
        pub fn settlement_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExpiryInfoSettlementDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.settlement_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for settlement_date: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ExpiryInfo> for super::ExpiryInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExpiryInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expiration_date: value.expiration_date?,
                expiry: value.expiry?,
                last_trading_day: value.last_trading_day?,
                settlement_date: value.settlement_date?,
            })
        }
    }
    impl ::std::convert::From<super::ExpiryInfo> for ExpiryInfo {
        fn from(value: super::ExpiryInfo) -> Self {
            Self {
                expiration_date: Ok(value.expiration_date),
                expiry: Ok(value.expiry),
                last_trading_day: Ok(value.last_trading_day),
                settlement_date: Ok(value.settlement_date),
            }
        }
    }
}
