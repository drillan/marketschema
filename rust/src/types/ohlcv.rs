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
#[doc = "ローソク足データを表現する（始値、高値、安値、終値、出来高）"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"OHLCV\","]
#[doc = "  \"description\": \"ローソク足データを表現する（始値、高値、安値、終値、出来高）\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"close\","]
#[doc = "    \"high\","]
#[doc = "    \"low\","]
#[doc = "    \"open\","]
#[doc = "    \"symbol\","]
#[doc = "    \"timestamp\","]
#[doc = "    \"volume\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"close\": {"]
#[doc = "      \"description\": \"価格\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"high\": {"]
#[doc = "      \"description\": \"価格\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"low\": {"]
#[doc = "      \"description\": \"価格\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"open\": {"]
#[doc = "      \"description\": \"価格\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"quote_volume\": {"]
#[doc = "      \"description\": \"売買代金（決済通貨建ての出来高）\","]
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
pub struct Ohlcv {
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    #[doc = "売買代金（決済通貨建ての出来高）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quote_volume: ::std::option::Option<f64>,
    #[doc = "銘柄識別子"]
    pub symbol: OhlcvSymbol,
    #[doc = "ISO 8601形式のタイムスタンプ (UTC)"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    pub volume: f64,
}
impl ::std::convert::From<&Ohlcv> for Ohlcv {
    fn from(value: &Ohlcv) -> Self {
        value.clone()
    }
}
impl Ohlcv {
    pub fn builder() -> builder::Ohlcv {
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
pub struct OhlcvSymbol(::std::string::String);
impl ::std::ops::Deref for OhlcvSymbol {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<OhlcvSymbol> for ::std::string::String {
    fn from(value: OhlcvSymbol) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OhlcvSymbol> for OhlcvSymbol {
    fn from(value: &OhlcvSymbol) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for OhlcvSymbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for OhlcvSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OhlcvSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OhlcvSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for OhlcvSymbol {
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
    pub struct Ohlcv {
        close: ::std::result::Result<f64, ::std::string::String>,
        high: ::std::result::Result<f64, ::std::string::String>,
        low: ::std::result::Result<f64, ::std::string::String>,
        open: ::std::result::Result<f64, ::std::string::String>,
        quote_volume: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        symbol: ::std::result::Result<super::OhlcvSymbol, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        volume: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for Ohlcv {
        fn default() -> Self {
            Self {
                close: Err("no value supplied for close".to_string()),
                high: Err("no value supplied for high".to_string()),
                low: Err("no value supplied for low".to_string()),
                open: Err("no value supplied for open".to_string()),
                quote_volume: Ok(Default::default()),
                symbol: Err("no value supplied for symbol".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                volume: Err("no value supplied for volume".to_string()),
            }
        }
    }
    impl Ohlcv {
        pub fn close<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.close = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for close: {}", e));
            self
        }
        pub fn high<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.high = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for high: {}", e));
            self
        }
        pub fn low<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.low = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for low: {}", e));
            self
        }
        pub fn open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open: {}", e));
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
            T: ::std::convert::TryInto<super::OhlcvSymbol>,
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
    impl ::std::convert::TryFrom<Ohlcv> for super::Ohlcv {
        type Error = super::error::ConversionError;
        fn try_from(value: Ohlcv) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                close: value.close?,
                high: value.high?,
                low: value.low?,
                open: value.open?,
                quote_volume: value.quote_volume?,
                symbol: value.symbol?,
                timestamp: value.timestamp?,
                volume: value.volume?,
            })
        }
    }
    impl ::std::convert::From<super::Ohlcv> for Ohlcv {
        fn from(value: super::Ohlcv) -> Self {
            Self {
                close: Ok(value.close),
                high: Ok(value.high),
                low: Ok(value.low),
                open: Ok(value.open),
                quote_volume: Ok(value.quote_volume),
                symbol: Ok(value.symbol),
                timestamp: Ok(value.timestamp),
                volume: Ok(value.volume),
            }
        }
    }
}
