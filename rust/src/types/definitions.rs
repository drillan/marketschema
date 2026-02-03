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
pub enum AssetClass {
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
impl ::std::convert::From<&Self> for AssetClass {
    fn from(value: &AssetClass) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AssetClass {
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
impl ::std::str::FromStr for AssetClass {
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
impl ::std::convert::TryFrom<&str> for AssetClass {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AssetClass {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssetClass {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "マーケットデータスキーマの共通型定義"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://marketschema.example.com/schemas/definitions\","]
#[doc = "  \"title\": \"Common Type Definitions\","]
#[doc = "  \"description\": \"マーケットデータスキーマの共通型定義\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CommonTypeDefinitions(pub ::serde_json::Value);
impl ::std::ops::Deref for CommonTypeDefinitions {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<CommonTypeDefinitions> for ::serde_json::Value {
    fn from(value: CommonTypeDefinitions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CommonTypeDefinitions> for CommonTypeDefinitions {
    fn from(value: &CommonTypeDefinitions) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for CommonTypeDefinitions {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
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
pub struct Currency(::std::string::String);
impl ::std::ops::Deref for Currency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Currency> for ::std::string::String {
    fn from(value: Currency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Currency> for Currency {
    fn from(value: &Currency) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Currency {
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
impl ::std::convert::TryFrom<&str> for Currency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Currency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Currency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Currency {
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
pub struct Date(::std::string::String);
impl ::std::ops::Deref for Date {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Date> for ::std::string::String {
    fn from(value: Date) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Date> for Date {
    fn from(value: &Date) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Date {
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
impl ::std::convert::TryFrom<&str> for Date {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Date {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Date {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Date {
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
pub struct Exchange(::std::string::String);
impl ::std::ops::Deref for Exchange {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Exchange> for ::std::string::String {
    fn from(value: Exchange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Exchange> for Exchange {
    fn from(value: &Exchange) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Exchange {
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
impl ::std::convert::TryFrom<&str> for Exchange {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Exchange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Exchange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Exchange {
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
#[doc = "オプション行使スタイル"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"オプション行使スタイル\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"american\","]
#[doc = "    \"european\","]
#[doc = "    \"bermudan\""]
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
pub enum ExerciseStyle {
    #[serde(rename = "american")]
    American,
    #[serde(rename = "european")]
    European,
    #[serde(rename = "bermudan")]
    Bermudan,
}
impl ::std::convert::From<&Self> for ExerciseStyle {
    fn from(value: &ExerciseStyle) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ExerciseStyle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::American => f.write_str("american"),
            Self::European => f.write_str("european"),
            Self::Bermudan => f.write_str("bermudan"),
        }
    }
}
impl ::std::str::FromStr for ExerciseStyle {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "american" => Ok(Self::American),
            "european" => Ok(Self::European),
            "bermudan" => Ok(Self::Bermudan),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExerciseStyle {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExerciseStyle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExerciseStyle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
pub struct ExpirySeries(::std::string::String);
impl ::std::ops::Deref for ExpirySeries {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExpirySeries> for ::std::string::String {
    fn from(value: ExpirySeries) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ExpirySeries> for ExpirySeries {
    fn from(value: &ExpirySeries) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ExpirySeries {
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
impl ::std::convert::TryFrom<&str> for ExpirySeries {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExpirySeries {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExpirySeries {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExpirySeries {
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
#[doc = "オプションタイプ（コール/プット）"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"オプションタイプ（コール/プット）\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"call\","]
#[doc = "    \"put\""]
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
pub enum OptionType {
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "put")]
    Put,
}
impl ::std::convert::From<&Self> for OptionType {
    fn from(value: &OptionType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for OptionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Call => f.write_str("call"),
            Self::Put => f.write_str("put"),
        }
    }
}
impl ::std::str::FromStr for OptionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "call" => Ok(Self::Call),
            "put" => Ok(Self::Put),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OptionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OptionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OptionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Price`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"価格\","]
#[doc = "  \"type\": \"number\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Price(pub f64);
impl ::std::ops::Deref for Price {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<Price> for f64 {
    fn from(value: Price) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Price> for Price {
    fn from(value: &Price) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for Price {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Price {
    type Err = <f64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Price {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Price {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Price {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Price {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
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
#[doc = "      \"description\": \"気配値\","]
#[doc = "      \"$ref\": \"#/$defs/Price\""]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"数量\","]
#[doc = "      \"$ref\": \"#/$defs/Size\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PriceLevel {
    #[doc = "気配値"]
    pub price: Price,
    #[doc = "数量"]
    pub size: Size,
}
impl ::std::convert::From<&PriceLevel> for PriceLevel {
    fn from(value: &PriceLevel) -> Self {
        value.clone()
    }
}
impl PriceLevel {
    pub fn builder() -> builder::PriceLevel {
        Default::default()
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
pub enum SettlementMethod {
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "physical")]
    Physical,
}
impl ::std::convert::From<&Self> for SettlementMethod {
    fn from(value: &SettlementMethod) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SettlementMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cash => f.write_str("cash"),
            Self::Physical => f.write_str("physical"),
        }
    }
}
impl ::std::str::FromStr for SettlementMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "cash" => Ok(Self::Cash),
            "physical" => Ok(Self::Physical),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SettlementMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SettlementMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SettlementMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
pub enum Side {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl ::std::convert::From<&Self> for Side {
    fn from(value: &Side) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Side {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Buy => f.write_str("buy"),
            Self::Sell => f.write_str("sell"),
        }
    }
}
impl ::std::str::FromStr for Side {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Side {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Side {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Side {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Size`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"数量\","]
#[doc = "  \"type\": \"number\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Size(pub f64);
impl ::std::ops::Deref for Size {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<Size> for f64 {
    fn from(value: Size) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Size> for Size {
    fn from(value: &Size) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for Size {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Size {
    type Err = <f64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Size {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Size {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Size {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Size {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
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
pub struct Symbol(::std::string::String);
impl ::std::ops::Deref for Symbol {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Symbol> for ::std::string::String {
    fn from(value: Symbol) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Symbol> for Symbol {
    fn from(value: &Symbol) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Symbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Symbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Symbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Symbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Symbol {
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
#[doc = "ISO 8601形式のタイムスタンプ (UTC)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ISO 8601形式のタイムスタンプ (UTC)\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"date-time\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Timestamp(pub ::chrono::DateTime<::chrono::offset::Utc>);
impl ::std::ops::Deref for Timestamp {
    type Target = ::chrono::DateTime<::chrono::offset::Utc>;
    fn deref(&self) -> &::chrono::DateTime<::chrono::offset::Utc> {
        &self.0
    }
}
impl ::std::convert::From<Timestamp> for ::chrono::DateTime<::chrono::offset::Utc> {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Timestamp> for Timestamp {
    fn from(value: &Timestamp) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::chrono::DateTime<::chrono::offset::Utc>> for Timestamp {
    fn from(value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Timestamp {
    type Err = <::chrono::DateTime<::chrono::offset::Utc> as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Timestamp {
    type Error = <::chrono::DateTime<::chrono::offset::Utc> as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Timestamp {
    type Error = <::chrono::DateTime<::chrono::offset::Utc> as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Timestamp {
    type Error = <::chrono::DateTime<::chrono::offset::Utc> as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
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
pub enum UnderlyingType {
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
impl ::std::convert::From<&Self> for UnderlyingType {
    fn from(value: &UnderlyingType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for UnderlyingType {
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
impl ::std::str::FromStr for UnderlyingType {
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
impl ::std::convert::TryFrom<&str> for UnderlyingType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UnderlyingType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UnderlyingType {
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
    pub struct PriceLevel {
        price: ::std::result::Result<super::Price, ::std::string::String>,
        size: ::std::result::Result<super::Size, ::std::string::String>,
    }
    impl ::std::default::Default for PriceLevel {
        fn default() -> Self {
            Self {
                price: Err("no value supplied for price".to_string()),
                size: Err("no value supplied for size".to_string()),
            }
        }
    }
    impl PriceLevel {
        pub fn price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Price>,
            T::Error: ::std::fmt::Display,
        {
            self.price = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for price: {}", e));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Size>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PriceLevel> for super::PriceLevel {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PriceLevel,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                price: value.price?,
                size: value.size?,
            })
        }
    }
    impl ::std::convert::From<super::PriceLevel> for PriceLevel {
        fn from(value: super::PriceLevel) -> Self {
            Self {
                price: Ok(value.price),
                size: Ok(value.size),
            }
        }
    }
}
