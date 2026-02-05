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
#[doc = "先物・オプション共通の情報を表現する"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"DerivativeInfo\","]
#[doc = "  \"description\": \"先物・オプション共通の情報を表現する\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"multiplier\","]
#[doc = "    \"tick_size\","]
#[doc = "    \"underlying_symbol\","]
#[doc = "    \"underlying_type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contract_value\": {"]
#[doc = "      \"description\": \"契約基本価値\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"contract_value_currency\": {"]
#[doc = "      \"description\": \"契約価値の通貨\","]
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
#[doc = "    \"is_inverse\": {"]
#[doc = "      \"description\": \"インバース契約か否か（false=linear、暗号資産デリバティブ向け）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"is_perpetual\": {"]
#[doc = "      \"description\": \"無期限契約か否か（暗号資産デリバティブ向け）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"lot_size\": {"]
#[doc = "      \"description\": \"取引単位（注文可能な最小数量単位）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"max_order_size\": {"]
#[doc = "      \"description\": \"最大注文数量\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"min_order_size\": {"]
#[doc = "      \"description\": \"最小注文数量\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"multiplier\": {"]
#[doc = "      \"description\": \"契約乗数（1契約あたりの乗数）\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"settlement_currency\": {"]
#[doc = "      \"description\": \"決済通貨\","]
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
#[doc = "    \"settlement_method\": {"]
#[doc = "      \"description\": \"決済方法\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"決済方法\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"cash\","]
#[doc = "            \"physical\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"settlement_price\": {"]
#[doc = "      \"description\": \"清算値段（証拠金計算・損益計算の基準）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tick_size\": {"]
#[doc = "      \"description\": \"呼値単位（最小価格変動）\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"tick_value\": {"]
#[doc = "      \"description\": \"ティック価値（1ティックあたりの金額変動）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"underlying_symbol\": {"]
#[doc = "      \"description\": \"銘柄識別子\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"underlying_type\": {"]
#[doc = "      \"description\": \"原資産タイプ\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"stock\","]
#[doc = "        \"index\","]
#[doc = "        \"etf\","]
#[doc = "        \"commodity\","]
#[doc = "        \"currency\","]
#[doc = "        \"crypto\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DerivativeInfo {
    #[doc = "契約基本価値"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub contract_value: ::std::option::Option<f64>,
    #[doc = "契約価値の通貨"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub contract_value_currency: ::std::option::Option<DerivativeInfoContractValueCurrency>,
    #[doc = "インバース契約か否か（false=linear、暗号資産デリバティブ向け）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_inverse: ::std::option::Option<bool>,
    #[doc = "無期限契約か否か（暗号資産デリバティブ向け）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_perpetual: ::std::option::Option<bool>,
    #[doc = "取引単位（注文可能な最小数量単位）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lot_size: ::std::option::Option<f64>,
    #[doc = "最大注文数量"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_order_size: ::std::option::Option<f64>,
    #[doc = "最小注文数量"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min_order_size: ::std::option::Option<f64>,
    pub multiplier: f64,
    #[doc = "決済通貨"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub settlement_currency: ::std::option::Option<DerivativeInfoSettlementCurrency>,
    #[doc = "決済方法"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub settlement_method: ::std::option::Option<DerivativeInfoSettlementMethod>,
    #[doc = "清算値段（証拠金計算・損益計算の基準）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub settlement_price: ::std::option::Option<f64>,
    pub tick_size: f64,
    #[doc = "ティック価値（1ティックあたりの金額変動）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tick_value: ::std::option::Option<f64>,
    #[doc = "銘柄識別子"]
    pub underlying_symbol: DerivativeInfoUnderlyingSymbol,
    #[doc = "原資産タイプ"]
    pub underlying_type: DerivativeInfoUnderlyingType,
}
impl ::std::convert::From<&DerivativeInfo> for DerivativeInfo {
    fn from(value: &DerivativeInfo) -> Self {
        value.clone()
    }
}
impl DerivativeInfo {
    pub fn builder() -> builder::DerivativeInfo {
        Default::default()
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
pub struct DerivativeInfoContractValueCurrency(::std::string::String);
impl ::std::ops::Deref for DerivativeInfoContractValueCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DerivativeInfoContractValueCurrency> for ::std::string::String {
    fn from(value: DerivativeInfoContractValueCurrency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DerivativeInfoContractValueCurrency>
    for DerivativeInfoContractValueCurrency
{
    fn from(value: &DerivativeInfoContractValueCurrency) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DerivativeInfoContractValueCurrency {
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
impl ::std::convert::TryFrom<&str> for DerivativeInfoContractValueCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DerivativeInfoContractValueCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DerivativeInfoContractValueCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DerivativeInfoContractValueCurrency {
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
pub struct DerivativeInfoSettlementCurrency(::std::string::String);
impl ::std::ops::Deref for DerivativeInfoSettlementCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DerivativeInfoSettlementCurrency> for ::std::string::String {
    fn from(value: DerivativeInfoSettlementCurrency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DerivativeInfoSettlementCurrency> for DerivativeInfoSettlementCurrency {
    fn from(value: &DerivativeInfoSettlementCurrency) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DerivativeInfoSettlementCurrency {
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
impl ::std::convert::TryFrom<&str> for DerivativeInfoSettlementCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DerivativeInfoSettlementCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DerivativeInfoSettlementCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DerivativeInfoSettlementCurrency {
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
#[doc = "決済方法"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"決済方法\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"cash\","]
#[doc = "    \"physical\""]
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
pub enum DerivativeInfoSettlementMethod {
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "physical")]
    Physical,
}
impl ::std::convert::From<&Self> for DerivativeInfoSettlementMethod {
    fn from(value: &DerivativeInfoSettlementMethod) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DerivativeInfoSettlementMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cash => f.write_str("cash"),
            Self::Physical => f.write_str("physical"),
        }
    }
}
impl ::std::str::FromStr for DerivativeInfoSettlementMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "cash" => Ok(Self::Cash),
            "physical" => Ok(Self::Physical),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DerivativeInfoSettlementMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DerivativeInfoSettlementMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DerivativeInfoSettlementMethod {
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
pub struct DerivativeInfoUnderlyingSymbol(::std::string::String);
impl ::std::ops::Deref for DerivativeInfoUnderlyingSymbol {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DerivativeInfoUnderlyingSymbol> for ::std::string::String {
    fn from(value: DerivativeInfoUnderlyingSymbol) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DerivativeInfoUnderlyingSymbol> for DerivativeInfoUnderlyingSymbol {
    fn from(value: &DerivativeInfoUnderlyingSymbol) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DerivativeInfoUnderlyingSymbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DerivativeInfoUnderlyingSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DerivativeInfoUnderlyingSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DerivativeInfoUnderlyingSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DerivativeInfoUnderlyingSymbol {
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
#[doc = "原資産タイプ"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"原資産タイプ\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"stock\","]
#[doc = "    \"index\","]
#[doc = "    \"etf\","]
#[doc = "    \"commodity\","]
#[doc = "    \"currency\","]
#[doc = "    \"crypto\""]
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
pub enum DerivativeInfoUnderlyingType {
    #[serde(rename = "stock")]
    Stock,
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "etf")]
    Etf,
    #[serde(rename = "commodity")]
    Commodity,
    #[serde(rename = "currency")]
    Currency,
    #[serde(rename = "crypto")]
    Crypto,
}
impl ::std::convert::From<&Self> for DerivativeInfoUnderlyingType {
    fn from(value: &DerivativeInfoUnderlyingType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DerivativeInfoUnderlyingType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Stock => f.write_str("stock"),
            Self::Index => f.write_str("index"),
            Self::Etf => f.write_str("etf"),
            Self::Commodity => f.write_str("commodity"),
            Self::Currency => f.write_str("currency"),
            Self::Crypto => f.write_str("crypto"),
        }
    }
}
impl ::std::str::FromStr for DerivativeInfoUnderlyingType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stock" => Ok(Self::Stock),
            "index" => Ok(Self::Index),
            "etf" => Ok(Self::Etf),
            "commodity" => Ok(Self::Commodity),
            "currency" => Ok(Self::Currency),
            "crypto" => Ok(Self::Crypto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DerivativeInfoUnderlyingType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DerivativeInfoUnderlyingType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DerivativeInfoUnderlyingType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DerivativeInfo {
        contract_value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        contract_value_currency: ::std::result::Result<
            ::std::option::Option<super::DerivativeInfoContractValueCurrency>,
            ::std::string::String,
        >,
        is_inverse: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_perpetual: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        lot_size: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        max_order_size: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        min_order_size: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        multiplier: ::std::result::Result<f64, ::std::string::String>,
        settlement_currency: ::std::result::Result<
            ::std::option::Option<super::DerivativeInfoSettlementCurrency>,
            ::std::string::String,
        >,
        settlement_method: ::std::result::Result<
            ::std::option::Option<super::DerivativeInfoSettlementMethod>,
            ::std::string::String,
        >,
        settlement_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tick_size: ::std::result::Result<f64, ::std::string::String>,
        tick_value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        underlying_symbol:
            ::std::result::Result<super::DerivativeInfoUnderlyingSymbol, ::std::string::String>,
        underlying_type:
            ::std::result::Result<super::DerivativeInfoUnderlyingType, ::std::string::String>,
    }
    impl ::std::default::Default for DerivativeInfo {
        fn default() -> Self {
            Self {
                contract_value: Ok(Default::default()),
                contract_value_currency: Ok(Default::default()),
                is_inverse: Ok(Default::default()),
                is_perpetual: Ok(Default::default()),
                lot_size: Ok(Default::default()),
                max_order_size: Ok(Default::default()),
                min_order_size: Ok(Default::default()),
                multiplier: Err("no value supplied for multiplier".to_string()),
                settlement_currency: Ok(Default::default()),
                settlement_method: Ok(Default::default()),
                settlement_price: Ok(Default::default()),
                tick_size: Err("no value supplied for tick_size".to_string()),
                tick_value: Ok(Default::default()),
                underlying_symbol: Err("no value supplied for underlying_symbol".to_string()),
                underlying_type: Err("no value supplied for underlying_type".to_string()),
            }
        }
    }
    impl DerivativeInfo {
        pub fn contract_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.contract_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contract_value: {}", e));
            self
        }
        pub fn contract_value_currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DerivativeInfoContractValueCurrency>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.contract_value_currency = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for contract_value_currency: {}",
                    e
                )
            });
            self
        }
        pub fn is_inverse<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_inverse = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_inverse: {}", e));
            self
        }
        pub fn is_perpetual<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_perpetual = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_perpetual: {}", e));
            self
        }
        pub fn lot_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lot_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lot_size: {}", e));
            self
        }
        pub fn max_order_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_order_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_order_size: {}", e));
            self
        }
        pub fn min_order_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_order_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_order_size: {}", e));
            self
        }
        pub fn multiplier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.multiplier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for multiplier: {}", e));
            self
        }
        pub fn settlement_currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DerivativeInfoSettlementCurrency>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.settlement_currency = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for settlement_currency: {}",
                    e
                )
            });
            self
        }
        pub fn settlement_method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DerivativeInfoSettlementMethod>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.settlement_method = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for settlement_method: {}",
                    e
                )
            });
            self
        }
        pub fn settlement_price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.settlement_price = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for settlement_price: {}",
                    e
                )
            });
            self
        }
        pub fn tick_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.tick_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tick_size: {}", e));
            self
        }
        pub fn tick_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.tick_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tick_value: {}", e));
            self
        }
        pub fn underlying_symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DerivativeInfoUnderlyingSymbol>,
            T::Error: ::std::fmt::Display,
        {
            self.underlying_symbol = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for underlying_symbol: {}",
                    e
                )
            });
            self
        }
        pub fn underlying_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DerivativeInfoUnderlyingType>,
            T::Error: ::std::fmt::Display,
        {
            self.underlying_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for underlying_type: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<DerivativeInfo> for super::DerivativeInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DerivativeInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                contract_value: value.contract_value?,
                contract_value_currency: value.contract_value_currency?,
                is_inverse: value.is_inverse?,
                is_perpetual: value.is_perpetual?,
                lot_size: value.lot_size?,
                max_order_size: value.max_order_size?,
                min_order_size: value.min_order_size?,
                multiplier: value.multiplier?,
                settlement_currency: value.settlement_currency?,
                settlement_method: value.settlement_method?,
                settlement_price: value.settlement_price?,
                tick_size: value.tick_size?,
                tick_value: value.tick_value?,
                underlying_symbol: value.underlying_symbol?,
                underlying_type: value.underlying_type?,
            })
        }
    }
    impl ::std::convert::From<super::DerivativeInfo> for DerivativeInfo {
        fn from(value: super::DerivativeInfo) -> Self {
            Self {
                contract_value: Ok(value.contract_value),
                contract_value_currency: Ok(value.contract_value_currency),
                is_inverse: Ok(value.is_inverse),
                is_perpetual: Ok(value.is_perpetual),
                lot_size: Ok(value.lot_size),
                max_order_size: Ok(value.max_order_size),
                min_order_size: Ok(value.min_order_size),
                multiplier: Ok(value.multiplier),
                settlement_currency: Ok(value.settlement_currency),
                settlement_method: Ok(value.settlement_method),
                settlement_price: Ok(value.settlement_price),
                tick_size: Ok(value.tick_size),
                tick_value: Ok(value.tick_value),
                underlying_symbol: Ok(value.underlying_symbol),
                underlying_type: Ok(value.underlying_type),
            }
        }
    }
}
