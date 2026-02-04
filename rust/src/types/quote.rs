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
#[doc = "最良気配値（BBO: Best Bid/Offer）を表現する"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Quote\","]
#[doc = "  \"description\": \"最良気配値（BBO: Best Bid/Offer）を表現する\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ask\","]
#[doc = "    \"bid\","]
#[doc = "    \"symbol\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ask\": {"]
#[doc = "      \"description\": \"価格\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"ask_size\": {"]
#[doc = "      \"description\": \"売り気配の数量\","]
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
#[doc = "    \"bid\": {"]
#[doc = "      \"description\": \"価格\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"bid_size\": {"]
#[doc = "      \"description\": \"買い気配の数量\","]
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
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Quote {
    pub ask: f64,
    #[doc = "売り気配の数量"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ask_size: ::std::option::Option<f64>,
    pub bid: f64,
    #[doc = "買い気配の数量"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bid_size: ::std::option::Option<f64>,
    #[doc = "銘柄識別子"]
    pub symbol: QuoteSymbol,
    #[doc = "ISO 8601形式のタイムスタンプ (UTC)"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Quote> for Quote {
    fn from(value: &Quote) -> Self {
        value.clone()
    }
}
impl Quote {
    pub fn builder() -> builder::Quote {
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
pub struct QuoteSymbol(::std::string::String);
impl ::std::ops::Deref for QuoteSymbol {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<QuoteSymbol> for ::std::string::String {
    fn from(value: QuoteSymbol) -> Self {
        value.0
    }
}
impl ::std::convert::From<&QuoteSymbol> for QuoteSymbol {
    fn from(value: &QuoteSymbol) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for QuoteSymbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for QuoteSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for QuoteSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QuoteSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for QuoteSymbol {
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
    pub struct Quote {
        ask: ::std::result::Result<f64, ::std::string::String>,
        ask_size: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        bid: ::std::result::Result<f64, ::std::string::String>,
        bid_size: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        symbol: ::std::result::Result<super::QuoteSymbol, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for Quote {
        fn default() -> Self {
            Self {
                ask: Err("no value supplied for ask".to_string()),
                ask_size: Ok(Default::default()),
                bid: Err("no value supplied for bid".to_string()),
                bid_size: Ok(Default::default()),
                symbol: Err("no value supplied for symbol".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl Quote {
        pub fn ask<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.ask = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ask: {}", e));
            self
        }
        pub fn ask_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ask_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ask_size: {}", e));
            self
        }
        pub fn bid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.bid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bid: {}", e));
            self
        }
        pub fn bid_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.bid_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bid_size: {}", e));
            self
        }
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::QuoteSymbol>,
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
    }
    impl ::std::convert::TryFrom<Quote> for super::Quote {
        type Error = super::error::ConversionError;
        fn try_from(value: Quote) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ask: value.ask?,
                ask_size: value.ask_size?,
                bid: value.bid?,
                bid_size: value.bid_size?,
                symbol: value.symbol?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::Quote> for Quote {
        fn from(value: super::Quote) -> Self {
            Self {
                ask: Ok(value.ask),
                ask_size: Ok(value.ask_size),
                bid: Ok(value.bid),
                bid_size: Ok(value.bid_size),
                symbol: Ok(value.symbol),
                timestamp: Ok(value.timestamp),
            }
        }
    }
}
