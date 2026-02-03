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
#[doc = "銘柄識別情報を表現する"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://marketschema.example.com/schemas/instrument\","]
#[doc = "  \"title\": \"Instrument\","]
#[doc = "  \"description\": \"銘柄識別情報を表現する\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"asset_class\","]
#[doc = "    \"symbol\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"asset_class\": {"]
#[doc = "      \"description\": \"資産クラス\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"equity\","]
#[doc = "        \"fund\","]
#[doc = "        \"bond\","]
#[doc = "        \"future\","]
#[doc = "        \"option\","]
#[doc = "        \"fx\","]
#[doc = "        \"crypto\","]
#[doc = "        \"cfd\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"base_currency\": {"]
#[doc = "      \"description\": \"基軸通貨（FX・暗号資産のペア商品）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"ISO 4217通貨コード（例: USD, JPY, EUR）\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[A-Z]{3}$\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"currency\": {"]
#[doc = "      \"description\": \"単一通貨（株式・債券等）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"ISO 4217通貨コード（例: USD, JPY, EUR）\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[A-Z]{3}$\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"exchange\": {"]
#[doc = "      \"description\": \"上場取引所（ISO 10383 MIC）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"ISO 10383市場識別コード（例: XJPX, XNYS）\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[A-Z]{4}$\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"quote_currency\": {"]
#[doc = "      \"description\": \"決済通貨（FX・暗号資産のペア商品）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"ISO 4217通貨コード（例: USD, JPY, EUR）\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[A-Z]{3}$\""]
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
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Instrument {
    #[doc = "資産クラス"]
    pub asset_class: InstrumentAssetClass,
    #[doc = "基軸通貨（FX・暗号資産のペア商品）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub base_currency: ::std::option::Option<InstrumentBaseCurrency>,
    #[doc = "単一通貨（株式・債券等）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<InstrumentCurrency>,
    #[doc = "上場取引所（ISO 10383 MIC）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exchange: ::std::option::Option<InstrumentExchange>,
    #[doc = "決済通貨（FX・暗号資産のペア商品）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quote_currency: ::std::option::Option<InstrumentQuoteCurrency>,
    #[doc = "銘柄識別子"]
    pub symbol: InstrumentSymbol,
}
impl ::std::convert::From<&Instrument> for Instrument {
    fn from(value: &Instrument) -> Self {
        value.clone()
    }
}
impl Instrument {
    pub fn builder() -> builder::Instrument {
        Default::default()
    }
}
#[doc = "資産クラス"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"資産クラス\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"equity\","]
#[doc = "    \"fund\","]
#[doc = "    \"bond\","]
#[doc = "    \"future\","]
#[doc = "    \"option\","]
#[doc = "    \"fx\","]
#[doc = "    \"crypto\","]
#[doc = "    \"cfd\""]
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
pub enum InstrumentAssetClass {
    #[serde(rename = "equity")]
    Equity,
    #[serde(rename = "fund")]
    Fund,
    #[serde(rename = "bond")]
    Bond,
    #[serde(rename = "future")]
    Future,
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "fx")]
    Fx,
    #[serde(rename = "crypto")]
    Crypto,
    #[serde(rename = "cfd")]
    Cfd,
}
impl ::std::convert::From<&Self> for InstrumentAssetClass {
    fn from(value: &InstrumentAssetClass) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for InstrumentAssetClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Equity => f.write_str("equity"),
            Self::Fund => f.write_str("fund"),
            Self::Bond => f.write_str("bond"),
            Self::Future => f.write_str("future"),
            Self::Option => f.write_str("option"),
            Self::Fx => f.write_str("fx"),
            Self::Crypto => f.write_str("crypto"),
            Self::Cfd => f.write_str("cfd"),
        }
    }
}
impl ::std::str::FromStr for InstrumentAssetClass {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "equity" => Ok(Self::Equity),
            "fund" => Ok(Self::Fund),
            "bond" => Ok(Self::Bond),
            "future" => Ok(Self::Future),
            "option" => Ok(Self::Option),
            "fx" => Ok(Self::Fx),
            "crypto" => Ok(Self::Crypto),
            "cfd" => Ok(Self::Cfd),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InstrumentAssetClass {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InstrumentAssetClass {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InstrumentAssetClass {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ISO 4217通貨コード（例: USD, JPY, EUR）"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ISO 4217通貨コード（例: USD, JPY, EUR）\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[A-Z]{3}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct InstrumentBaseCurrency(::std::string::String);
impl ::std::ops::Deref for InstrumentBaseCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InstrumentBaseCurrency> for ::std::string::String {
    fn from(value: InstrumentBaseCurrency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InstrumentBaseCurrency> for InstrumentBaseCurrency {
    fn from(value: &InstrumentBaseCurrency) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for InstrumentBaseCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Z]{3}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Z]{3}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InstrumentBaseCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InstrumentBaseCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InstrumentBaseCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InstrumentBaseCurrency {
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
#[doc = "ISO 4217通貨コード（例: USD, JPY, EUR）"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ISO 4217通貨コード（例: USD, JPY, EUR）\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[A-Z]{3}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct InstrumentCurrency(::std::string::String);
impl ::std::ops::Deref for InstrumentCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InstrumentCurrency> for ::std::string::String {
    fn from(value: InstrumentCurrency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InstrumentCurrency> for InstrumentCurrency {
    fn from(value: &InstrumentCurrency) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for InstrumentCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Z]{3}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Z]{3}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InstrumentCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InstrumentCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InstrumentCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InstrumentCurrency {
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
#[doc = "ISO 10383市場識別コード（例: XJPX, XNYS）"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ISO 10383市場識別コード（例: XJPX, XNYS）\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[A-Z]{4}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct InstrumentExchange(::std::string::String);
impl ::std::ops::Deref for InstrumentExchange {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InstrumentExchange> for ::std::string::String {
    fn from(value: InstrumentExchange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InstrumentExchange> for InstrumentExchange {
    fn from(value: &InstrumentExchange) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for InstrumentExchange {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Z]{4}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Z]{4}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InstrumentExchange {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InstrumentExchange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InstrumentExchange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InstrumentExchange {
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
#[doc = "ISO 4217通貨コード（例: USD, JPY, EUR）"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ISO 4217通貨コード（例: USD, JPY, EUR）\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[A-Z]{3}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct InstrumentQuoteCurrency(::std::string::String);
impl ::std::ops::Deref for InstrumentQuoteCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InstrumentQuoteCurrency> for ::std::string::String {
    fn from(value: InstrumentQuoteCurrency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InstrumentQuoteCurrency> for InstrumentQuoteCurrency {
    fn from(value: &InstrumentQuoteCurrency) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for InstrumentQuoteCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Z]{3}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Z]{3}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InstrumentQuoteCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InstrumentQuoteCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InstrumentQuoteCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InstrumentQuoteCurrency {
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
pub struct InstrumentSymbol(::std::string::String);
impl ::std::ops::Deref for InstrumentSymbol {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InstrumentSymbol> for ::std::string::String {
    fn from(value: InstrumentSymbol) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InstrumentSymbol> for InstrumentSymbol {
    fn from(value: &InstrumentSymbol) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for InstrumentSymbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InstrumentSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InstrumentSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InstrumentSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InstrumentSymbol {
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
    pub struct Instrument {
        asset_class: ::std::result::Result<super::InstrumentAssetClass, ::std::string::String>,
        base_currency: ::std::result::Result<
            ::std::option::Option<super::InstrumentBaseCurrency>,
            ::std::string::String,
        >,
        currency: ::std::result::Result<
            ::std::option::Option<super::InstrumentCurrency>,
            ::std::string::String,
        >,
        exchange: ::std::result::Result<
            ::std::option::Option<super::InstrumentExchange>,
            ::std::string::String,
        >,
        quote_currency: ::std::result::Result<
            ::std::option::Option<super::InstrumentQuoteCurrency>,
            ::std::string::String,
        >,
        symbol: ::std::result::Result<super::InstrumentSymbol, ::std::string::String>,
    }
    impl ::std::default::Default for Instrument {
        fn default() -> Self {
            Self {
                asset_class: Err("no value supplied for asset_class".to_string()),
                base_currency: Ok(Default::default()),
                currency: Ok(Default::default()),
                exchange: Ok(Default::default()),
                quote_currency: Ok(Default::default()),
                symbol: Err("no value supplied for symbol".to_string()),
            }
        }
    }
    impl Instrument {
        pub fn asset_class<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InstrumentAssetClass>,
            T::Error: ::std::fmt::Display,
        {
            self.asset_class = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for asset_class: {}", e));
            self
        }
        pub fn base_currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InstrumentBaseCurrency>>,
            T::Error: ::std::fmt::Display,
        {
            self.base_currency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_currency: {}", e));
            self
        }
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InstrumentCurrency>>,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for currency: {}", e));
            self
        }
        pub fn exchange<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InstrumentExchange>>,
            T::Error: ::std::fmt::Display,
        {
            self.exchange = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exchange: {}", e));
            self
        }
        pub fn quote_currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InstrumentQuoteCurrency>>,
            T::Error: ::std::fmt::Display,
        {
            self.quote_currency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for quote_currency: {}", e));
            self
        }
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InstrumentSymbol>,
            T::Error: ::std::fmt::Display,
        {
            self.symbol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbol: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Instrument> for super::Instrument {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Instrument,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                asset_class: value.asset_class?,
                base_currency: value.base_currency?,
                currency: value.currency?,
                exchange: value.exchange?,
                quote_currency: value.quote_currency?,
                symbol: value.symbol?,
            })
        }
    }
    impl ::std::convert::From<super::Instrument> for Instrument {
        fn from(value: super::Instrument) -> Self {
            Self {
                asset_class: Ok(value.asset_class),
                base_currency: Ok(value.base_currency),
                currency: Ok(value.currency),
                exchange: Ok(value.exchange),
                quote_currency: Ok(value.quote_currency),
                symbol: Ok(value.symbol),
            }
        }
    }
}
