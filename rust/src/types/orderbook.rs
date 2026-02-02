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
#[doc = "複数レベルの板情報を表現する"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://marketschema.example.com/schemas/orderbook\","]
#[doc = "  \"title\": \"OrderBook\","]
#[doc = "  \"description\": \"複数レベルの板情報を表現する\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"asks\","]
#[doc = "    \"bids\","]
#[doc = "    \"symbol\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"asks\": {"]
#[doc = "      \"description\": \"売り板（価格昇順）\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"板情報の気配レベル\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"price\","]
#[doc = "          \"size\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"price\": {"]
#[doc = "            \"description\": \"価格\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"size\": {"]
#[doc = "            \"description\": \"数量\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"unevaluatedProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"bids\": {"]
#[doc = "      \"description\": \"買い板（価格降順）\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"板情報の気配レベル\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"price\","]
#[doc = "          \"size\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"price\": {"]
#[doc = "            \"description\": \"価格\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"size\": {"]
#[doc = "            \"description\": \"数量\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"unevaluatedProperties\": false"]
#[doc = "      }"]
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
pub struct OrderBook {
    #[doc = "売り板（価格昇順）"]
    pub asks: ::std::vec::Vec<OrderBookAsksItem>,
    #[doc = "買い板（価格降順）"]
    pub bids: ::std::vec::Vec<OrderBookBidsItem>,
    #[doc = "銘柄識別子"]
    pub symbol: OrderBookSymbol,
    #[doc = "ISO 8601形式のタイムスタンプ (UTC)"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&OrderBook> for OrderBook {
    fn from(value: &OrderBook) -> Self {
        value.clone()
    }
}
impl OrderBook {
    pub fn builder() -> builder::OrderBook {
        Default::default()
    }
}
#[doc = "板情報の気配レベル"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"板情報の気配レベル\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"price\","]
#[doc = "    \"size\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"price\": {"]
#[doc = "      \"description\": \"価格\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"数量\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OrderBookAsksItem {
    pub price: f64,
    pub size: f64,
}
impl ::std::convert::From<&OrderBookAsksItem> for OrderBookAsksItem {
    fn from(value: &OrderBookAsksItem) -> Self {
        value.clone()
    }
}
impl OrderBookAsksItem {
    pub fn builder() -> builder::OrderBookAsksItem {
        Default::default()
    }
}
#[doc = "板情報の気配レベル"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"板情報の気配レベル\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"price\","]
#[doc = "    \"size\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"price\": {"]
#[doc = "      \"description\": \"価格\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"数量\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OrderBookBidsItem {
    pub price: f64,
    pub size: f64,
}
impl ::std::convert::From<&OrderBookBidsItem> for OrderBookBidsItem {
    fn from(value: &OrderBookBidsItem) -> Self {
        value.clone()
    }
}
impl OrderBookBidsItem {
    pub fn builder() -> builder::OrderBookBidsItem {
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
pub struct OrderBookSymbol(::std::string::String);
impl ::std::ops::Deref for OrderBookSymbol {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<OrderBookSymbol> for ::std::string::String {
    fn from(value: OrderBookSymbol) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OrderBookSymbol> for OrderBookSymbol {
    fn from(value: &OrderBookSymbol) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for OrderBookSymbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for OrderBookSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OrderBookSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrderBookSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for OrderBookSymbol {
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
    pub struct OrderBook {
        asks:
            ::std::result::Result<::std::vec::Vec<super::OrderBookAsksItem>, ::std::string::String>,
        bids:
            ::std::result::Result<::std::vec::Vec<super::OrderBookBidsItem>, ::std::string::String>,
        symbol: ::std::result::Result<super::OrderBookSymbol, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for OrderBook {
        fn default() -> Self {
            Self {
                asks: Err("no value supplied for asks".to_string()),
                bids: Err("no value supplied for bids".to_string()),
                symbol: Err("no value supplied for symbol".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl OrderBook {
        pub fn asks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrderBookAsksItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.asks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for asks: {}", e));
            self
        }
        pub fn bids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrderBookBidsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.bids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bids: {}", e));
            self
        }
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrderBookSymbol>,
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
    impl ::std::convert::TryFrom<OrderBook> for super::OrderBook {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrderBook,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                asks: value.asks?,
                bids: value.bids?,
                symbol: value.symbol?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::OrderBook> for OrderBook {
        fn from(value: super::OrderBook) -> Self {
            Self {
                asks: Ok(value.asks),
                bids: Ok(value.bids),
                symbol: Ok(value.symbol),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrderBookAsksItem {
        price: ::std::result::Result<f64, ::std::string::String>,
        size: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for OrderBookAsksItem {
        fn default() -> Self {
            Self {
                price: Err("no value supplied for price".to_string()),
                size: Err("no value supplied for size".to_string()),
            }
        }
    }
    impl OrderBookAsksItem {
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
    }
    impl ::std::convert::TryFrom<OrderBookAsksItem> for super::OrderBookAsksItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrderBookAsksItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                price: value.price?,
                size: value.size?,
            })
        }
    }
    impl ::std::convert::From<super::OrderBookAsksItem> for OrderBookAsksItem {
        fn from(value: super::OrderBookAsksItem) -> Self {
            Self {
                price: Ok(value.price),
                size: Ok(value.size),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrderBookBidsItem {
        price: ::std::result::Result<f64, ::std::string::String>,
        size: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for OrderBookBidsItem {
        fn default() -> Self {
            Self {
                price: Err("no value supplied for price".to_string()),
                size: Err("no value supplied for size".to_string()),
            }
        }
    }
    impl OrderBookBidsItem {
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
    }
    impl ::std::convert::TryFrom<OrderBookBidsItem> for super::OrderBookBidsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrderBookBidsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                price: value.price?,
                size: value.size?,
            })
        }
    }
    impl ::std::convert::From<super::OrderBookBidsItem> for OrderBookBidsItem {
        fn from(value: super::OrderBookBidsItem) -> Self {
            Self {
                price: Ok(value.price),
                size: Ok(value.size),
            }
        }
    }
}
