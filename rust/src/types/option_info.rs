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
#[doc = "オプション固有の情報を表現する"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://marketschema.example.com/schemas/option_info\","]
#[doc = "  \"title\": \"OptionInfo\","]
#[doc = "  \"description\": \"オプション固有の情報を表現する\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"option_type\","]
#[doc = "    \"strike_price\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"exercise_style\": {"]
#[doc = "      \"description\": \"行使スタイル（american/european/bermudan）\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"オプション行使スタイル\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"american\","]
#[doc = "            \"european\","]
#[doc = "            \"bermudan\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"option_type\": {"]
#[doc = "      \"description\": \"オプションタイプ（コール/プット）\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"call\","]
#[doc = "        \"put\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"strike_price\": {"]
#[doc = "      \"description\": \"価格\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OptionInfo {
    #[doc = "行使スタイル（american/european/bermudan）"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exercise_style: ::std::option::Option<OptionInfoExerciseStyle>,
    #[doc = "オプションタイプ（コール/プット）"]
    pub option_type: OptionInfoOptionType,
    pub strike_price: f64,
}
impl ::std::convert::From<&OptionInfo> for OptionInfo {
    fn from(value: &OptionInfo) -> Self {
        value.clone()
    }
}
impl OptionInfo {
    pub fn builder() -> builder::OptionInfo {
        Default::default()
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
pub enum OptionInfoExerciseStyle {
    #[serde(rename = "american")]
    American,
    #[serde(rename = "european")]
    European,
    #[serde(rename = "bermudan")]
    Bermudan,
}
impl ::std::convert::From<&Self> for OptionInfoExerciseStyle {
    fn from(value: &OptionInfoExerciseStyle) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for OptionInfoExerciseStyle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::American => f.write_str("american"),
            Self::European => f.write_str("european"),
            Self::Bermudan => f.write_str("bermudan"),
        }
    }
}
impl ::std::str::FromStr for OptionInfoExerciseStyle {
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
impl ::std::convert::TryFrom<&str> for OptionInfoExerciseStyle {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OptionInfoExerciseStyle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OptionInfoExerciseStyle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
pub enum OptionInfoOptionType {
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "put")]
    Put,
}
impl ::std::convert::From<&Self> for OptionInfoOptionType {
    fn from(value: &OptionInfoOptionType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for OptionInfoOptionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Call => f.write_str("call"),
            Self::Put => f.write_str("put"),
        }
    }
}
impl ::std::str::FromStr for OptionInfoOptionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "call" => Ok(Self::Call),
            "put" => Ok(Self::Put),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OptionInfoOptionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OptionInfoOptionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OptionInfoOptionType {
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
    pub struct OptionInfo {
        exercise_style: ::std::result::Result<
            ::std::option::Option<super::OptionInfoExerciseStyle>,
            ::std::string::String,
        >,
        option_type: ::std::result::Result<super::OptionInfoOptionType, ::std::string::String>,
        strike_price: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for OptionInfo {
        fn default() -> Self {
            Self {
                exercise_style: Ok(Default::default()),
                option_type: Err("no value supplied for option_type".to_string()),
                strike_price: Err("no value supplied for strike_price".to_string()),
            }
        }
    }
    impl OptionInfo {
        pub fn exercise_style<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OptionInfoExerciseStyle>>,
            T::Error: ::std::fmt::Display,
        {
            self.exercise_style = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exercise_style: {}", e));
            self
        }
        pub fn option_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OptionInfoOptionType>,
            T::Error: ::std::fmt::Display,
        {
            self.option_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for option_type: {}", e));
            self
        }
        pub fn strike_price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.strike_price = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strike_price: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<OptionInfo> for super::OptionInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OptionInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                exercise_style: value.exercise_style?,
                option_type: value.option_type?,
                strike_price: value.strike_price?,
            })
        }
    }
    impl ::std::convert::From<super::OptionInfo> for OptionInfo {
        fn from(value: super::OptionInfo) -> Self {
            Self {
                exercise_style: Ok(value.exercise_style),
                option_type: Ok(value.option_type),
                strike_price: Ok(value.strike_price),
            }
        }
    }
}
