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
#[doc = "個別約定（歩み値 / Time & Sales）を表現する"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://marketschema.example.com/schemas/trade\","]
#[doc = "  \"title\": \"Trade\","]
#[doc = "  \"description\": \"個別約定（歩み値 / Time & Sales）を表現する\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"price\","]
#[doc = "    \"side\","]
#[doc = "    \"size\","]
#[doc = "    \"symbol\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"price\": {"]
#[doc = "      \"description\": \"価格\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"side\": {"]
#[doc = "      \"description\": \"売買方向\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"buy\","]
#[doc = "        \"sell\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"数量\","]
#[doc = "      \"type\": \"number\""]
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
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Trade {
    pub price: f64,
    #[doc = "売買方向"]
    pub side: TradeSide,
    pub size: f64,
    #[doc = "銘柄識別子"]
    pub symbol: TradeSymbol,
    #[doc = "ISO 8601形式のタイムスタンプ (UTC)"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Trade> for Trade {
    fn from(value: &Trade) -> Self {
        value.clone()
    }
}
impl Trade {
    pub fn builder() -> builder::Trade {
        Default::default()
    }
}
#[doc = "売買方向"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"売買方向\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"buy\","]
#[doc = "    \"sell\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TradeSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl ::std::convert::From<&Self> for TradeSide {
    fn from(value: &TradeSide) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TradeSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Buy => f.write_str("buy"),
            Self::Sell => f.write_str("sell"),
        }
    }
}
impl ::std::str::FromStr for TradeSide {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TradeSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TradeSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TradeSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
pub struct TradeSymbol(::std::string::String);
impl ::std::ops::Deref for TradeSymbol {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TradeSymbol> for ::std::string::String {
    fn from(value: TradeSymbol) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TradeSymbol> for TradeSymbol {
    fn from(value: &TradeSymbol) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TradeSymbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TradeSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TradeSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TradeSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TradeSymbol {
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
    pub struct Trade {
        price: ::std::result::Result<f64, ::std::string::String>,
        side: ::std::result::Result<super::TradeSide, ::std::string::String>,
        size: ::std::result::Result<f64, ::std::string::String>,
        symbol: ::std::result::Result<super::TradeSymbol, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for Trade {
        fn default() -> Self {
            Self {
                price: Err("no value supplied for price".to_string()),
                side: Err("no value supplied for side".to_string()),
                size: Err("no value supplied for size".to_string()),
                symbol: Err("no value supplied for symbol".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl Trade {
        pub fn price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.price = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for price: {}", e));
            self
        }
        pub fn side<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TradeSide>,
            T::Error: ::std::fmt::Display,
        {
            self.side = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for side: {}", e));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TradeSymbol>,
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
    impl ::std::convert::TryFrom<Trade> for super::Trade {
        type Error = super::error::ConversionError;
        fn try_from(value: Trade) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                price: value.price?,
                side: value.side?,
                size: value.size?,
                symbol: value.symbol?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::Trade> for Trade {
        fn from(value: super::Trade) -> Self {
            Self {
                price: Ok(value.price),
                side: Ok(value.side),
                size: Ok(value.size),
                symbol: Ok(value.symbol),
                timestamp: Ok(value.timestamp),
            }
        }
    }
}
