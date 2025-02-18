#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
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
#[doc = "Amount"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"amount\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"assetCode\","]
#[doc = "    \"assetScale\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"assetCode\": {"]
#[doc = "      \"$ref\": \"#/$defs/assetCode\""]
#[doc = "    },"]
#[doc = "    \"assetScale\": {"]
#[doc = "      \"$ref\": \"#/$defs/assetScale\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The value is an unsigned 64-bit integer amount, represented as a string.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uint64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Amount {
    #[serde(rename = "assetCode")]
    pub asset_code: AssetCode,
    #[serde(rename = "assetScale")]
    pub asset_scale: AssetScale,
    #[doc = "The value is an unsigned 64-bit integer amount, represented as a string."]
    pub value: ::std::string::String,
}
impl ::std::convert::From<&Amount> for Amount {
    fn from(value: &Amount) -> Self {
        value.clone()
    }
}
#[doc = "The assetCode is a code that indicates the underlying asset. This SHOULD be an ISO4217 currency code."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Asset code\","]
#[doc = "  \"description\": \"The assetCode is a code that indicates the underlying asset. This SHOULD be an ISO4217 currency code.\","]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct AssetCode(pub ::std::string::String);
impl ::std::ops::Deref for AssetCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AssetCode> for ::std::string::String {
    fn from(value: AssetCode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AssetCode> for AssetCode {
    fn from(value: &AssetCode) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for AssetCode {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for AssetCode {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for AssetCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "The scale of amounts denoted in the corresponding asset code."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Asset scale\","]
#[doc = "  \"description\": \"The scale of amounts denoted in the corresponding asset code.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"maximum\": 255.0,"]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct AssetScale(pub u8);
impl ::std::ops::Deref for AssetScale {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<AssetScale> for u8 {
    fn from(value: AssetScale) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AssetScale> for AssetScale {
    fn from(value: &AssetScale) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<u8> for AssetScale {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for AssetScale {
    type Err = <u8 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for AssetScale {
    type Error = <u8 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for AssetScale {
    type Error = <u8 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for AssetScale {
    type Error = <u8 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for AssetScale {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "IlpPaymentMethod"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"examples\": ["]
#[doc = "    {"]
#[doc = "      \"ilpAddress\": \"string\","]
#[doc = "      \"sharedSecret\": \"string\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ilpAddress\","]
#[doc = "    \"sharedSecret\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ilpAddress\": {"]
#[doc = "      \"description\": \"The ILP address to use when establishing a STREAM connection.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 1023,"]
#[doc = "      \"pattern\": \"^(g|private|example|peer|self|test[1-3]?|local)([.][a-zA-Z0-9_~-]+)+$\""]
#[doc = "    },"]
#[doc = "    \"sharedSecret\": {"]
#[doc = "      \"description\": \"The base64 url-encoded shared secret to use when establishing a STREAM connection.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[a-zA-Z0-9-_]+$\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"ilp\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct IlpPaymentMethod {
    #[doc = "The ILP address to use when establishing a STREAM connection."]
    #[serde(rename = "ilpAddress")]
    pub ilp_address: IlpPaymentMethodIlpAddress,
    #[doc = "The base64 url-encoded shared secret to use when establishing a STREAM connection."]
    #[serde(rename = "sharedSecret")]
    pub shared_secret: IlpPaymentMethodSharedSecret,
    #[serde(rename = "type")]
    pub type_: IlpPaymentMethodType,
}
impl ::std::convert::From<&IlpPaymentMethod> for IlpPaymentMethod {
    fn from(value: &IlpPaymentMethod) -> Self {
        value.clone()
    }
}
#[doc = "The ILP address to use when establishing a STREAM connection."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The ILP address to use when establishing a STREAM connection.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 1023,"]
#[doc = "  \"pattern\": \"^(g|private|example|peer|self|test[1-3]?|local)([.][a-zA-Z0-9_~-]+)+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct IlpPaymentMethodIlpAddress(::std::string::String);
impl ::std::ops::Deref for IlpPaymentMethodIlpAddress {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<IlpPaymentMethodIlpAddress> for ::std::string::String {
    fn from(value: IlpPaymentMethodIlpAddress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IlpPaymentMethodIlpAddress> for IlpPaymentMethodIlpAddress {
    fn from(value: &IlpPaymentMethodIlpAddress) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for IlpPaymentMethodIlpAddress {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.len() > 1023usize {
            return Err("longer than 1023 characters".into());
        }
        if regress::Regex::new(
            "^(g|private|example|peer|self|test[1-3]?|local)([.][a-zA-Z0-9_~-]+)+$",
        )
        .unwrap()
        .find(value)
        .is_none()
        {
            return Err ("doesn't match pattern \"^(g|private|example|peer|self|test[1-3]?|local)([.][a-zA-Z0-9_~-]+)+$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for IlpPaymentMethodIlpAddress {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IlpPaymentMethodIlpAddress {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IlpPaymentMethodIlpAddress {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for IlpPaymentMethodIlpAddress {
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
#[doc = "The base64 url-encoded shared secret to use when establishing a STREAM connection."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The base64 url-encoded shared secret to use when establishing a STREAM connection.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z0-9-_]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct IlpPaymentMethodSharedSecret(::std::string::String);
impl ::std::ops::Deref for IlpPaymentMethodSharedSecret {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<IlpPaymentMethodSharedSecret> for ::std::string::String {
    fn from(value: IlpPaymentMethodSharedSecret) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IlpPaymentMethodSharedSecret> for IlpPaymentMethodSharedSecret {
    fn from(value: &IlpPaymentMethodSharedSecret) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for IlpPaymentMethodSharedSecret {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[a-zA-Z0-9-_]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[a-zA-Z0-9-_]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for IlpPaymentMethodSharedSecret {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IlpPaymentMethodSharedSecret {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IlpPaymentMethodSharedSecret {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for IlpPaymentMethodSharedSecret {
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
#[doc = "IlpPaymentMethodType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ilp\""]
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
pub enum IlpPaymentMethodType {
    #[serde(rename = "ilp")]
    Ilp,
}
impl ::std::convert::From<&Self> for IlpPaymentMethodType {
    fn from(value: &IlpPaymentMethodType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for IlpPaymentMethodType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ilp => write!(f, "ilp"),
        }
    }
}
impl ::std::str::FromStr for IlpPaymentMethodType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ilp" => Ok(Self::Ilp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IlpPaymentMethodType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IlpPaymentMethodType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IlpPaymentMethodType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An **incoming payment** resource represents a payment that will be, is currently being, or has been received by the account."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Incoming Payment\","]
#[doc = "  \"description\": \"An **incoming payment** resource represents a payment that will be, is currently being, or has been received by the account.\","]
#[doc = "  \"examples\": ["]
#[doc = "    {"]
#[doc = "      \"completed\": true,"]
#[doc = "      \"createdAt\": \"2022-03-12T23:20:50.52Z\","]
#[doc = "      \"expiresAt\": \"2022-04-12T23:20:50.52Z\","]
#[doc = "      \"id\": \"https://ilp.rafiki.money/incoming-payments/016da9d5-c9a4-4c80-a354-86b915a04ff8\","]
#[doc = "      \"incomingAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"250\""]
#[doc = "      },"]
#[doc = "      \"metadata\": {"]
#[doc = "        \"description\": \"Hi Mo, this is for the cappuccino I bought for you the other day.\","]
#[doc = "        \"externalRef\": \"Coffee w/ Mo on 10 March 22\""]
#[doc = "      },"]
#[doc = "      \"receivedAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"250\""]
#[doc = "      },"]
#[doc = "      \"updatedAt\": \"2022-04-01T10:24:36.11Z\","]
#[doc = "      \"walletAddress\": \"https://ilp.rafiki.money/alice/\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"createdAt\": \"2022-03-12T23:20:50.52Z\","]
#[doc = "      \"expiresAt\": \"2022-04-12T23:20:50.52Z\","]
#[doc = "      \"id\": \"https://ilp.rafiki.money/incoming-payments/456da9d5-c9a4-4c80-a354-86b915a04ff8\","]
#[doc = "      \"incomingAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"2500\""]
#[doc = "      },"]
#[doc = "      \"receivedAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"0\""]
#[doc = "      },"]
#[doc = "      \"updatedAt\": \"2022-03-12T23:20:50.52Z\","]
#[doc = "      \"walletAddress\": \"https://ilp.rafiki.money/alice/\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"completed\","]
#[doc = "    \"createdAt\","]
#[doc = "    \"id\","]
#[doc = "    \"receivedAmount\","]
#[doc = "    \"updatedAt\","]
#[doc = "    \"walletAddress\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"completed\": {"]
#[doc = "      \"description\": \"Describes whether the incoming payment has completed receiving fund.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"createdAt\": {"]
#[doc = "      \"description\": \"The date and time when the incoming payment was created.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"expiresAt\": {"]
#[doc = "      \"description\": \"The date and time when payments under this incoming payment will no longer be accepted.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The URL identifying the incoming payment.\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"incomingAmount\": {"]
#[doc = "      \"description\": \"The maximum amount that should be paid into the wallet address under this incoming payment.\","]
#[doc = "      \"$ref\": \"#/$defs/amount\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Additional metadata associated with the incoming payment. (Optional)\","]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"receivedAmount\": {"]
#[doc = "      \"description\": \"The total amount that has been paid into the wallet address under this incoming payment.\","]
#[doc = "      \"$ref\": \"#/$defs/amount\""]
#[doc = "    },"]
#[doc = "    \"updatedAt\": {"]
#[doc = "      \"description\": \"The date and time when the incoming payment was updated.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"walletAddress\": {"]
#[doc = "      \"description\": \"The URL of the wallet address this payment is being made into.\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct IncomingPayment {
    #[doc = "Describes whether the incoming payment has completed receiving fund."]
    pub completed: bool,
    #[doc = "The date and time when the incoming payment was created."]
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The date and time when payments under this incoming payment will no longer be accepted."]
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "The URL identifying the incoming payment."]
    pub id: ::std::string::String,
    #[doc = "The maximum amount that should be paid into the wallet address under this incoming payment."]
    #[serde(
        rename = "incomingAmount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub incoming_amount: ::std::option::Option<Amount>,
    #[doc = "Additional metadata associated with the incoming payment. (Optional)"]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The total amount that has been paid into the wallet address under this incoming payment."]
    #[serde(rename = "receivedAmount")]
    pub received_amount: Amount,
    #[doc = "The date and time when the incoming payment was updated."]
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The URL of the wallet address this payment is being made into."]
    #[serde(rename = "walletAddress")]
    pub wallet_address: ::std::string::String,
}
impl ::std::convert::From<&IncomingPayment> for IncomingPayment {
    fn from(value: &IncomingPayment) -> Self {
        value.clone()
    }
}
#[doc = "An **incoming payment** resource with public details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Incoming Payment with payment methods\","]
#[doc = "  \"description\": \"An **incoming payment** resource with public details.\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/incoming-payment\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"methods\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"methods\": {"]
#[doc = "          \"description\": \"The list of payment methods supported by this incoming payment.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"anyOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/ilp-payment-method\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"minItems\": 0,"]
#[doc = "          \"uniqueItems\": true"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct IncomingPaymentWithMethods {
    #[doc = "Describes whether the incoming payment has completed receiving fund."]
    pub completed: bool,
    #[doc = "The date and time when the incoming payment was created."]
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The date and time when payments under this incoming payment will no longer be accepted."]
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "The URL identifying the incoming payment."]
    pub id: ::std::string::String,
    #[doc = "The maximum amount that should be paid into the wallet address under this incoming payment."]
    #[serde(
        rename = "incomingAmount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub incoming_amount: ::std::option::Option<Amount>,
    #[doc = "Additional metadata associated with the incoming payment. (Optional)"]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The list of payment methods supported by this incoming payment."]
    pub methods: Vec<IlpPaymentMethod>,
    #[doc = "The total amount that has been paid into the wallet address under this incoming payment."]
    #[serde(rename = "receivedAmount")]
    pub received_amount: Amount,
    #[doc = "The date and time when the incoming payment was updated."]
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The URL of the wallet address this payment is being made into."]
    #[serde(rename = "walletAddress")]
    pub wallet_address: ::std::string::String,
}
impl ::std::convert::From<&IncomingPaymentWithMethods> for IncomingPaymentWithMethods {
    fn from(value: &IncomingPaymentWithMethods) -> Self {
        value.clone()
    }
}
#[doc = "An **outgoing payment** resource represents a payment that will be, is currently being, or has previously been, sent from the wallet address."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Outgoing Payment\","]
#[doc = "  \"description\": \"An **outgoing payment** resource represents a payment that will be, is currently being, or has previously been, sent from the wallet address.\","]
#[doc = "  \"examples\": ["]
#[doc = "    {"]
#[doc = "      \"createdAt\": \"2022-03-12T23:20:50.52Z\","]
#[doc = "      \"debitAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"2600\""]
#[doc = "      },"]
#[doc = "      \"failed\": false,"]
#[doc = "      \"id\": \"https://ilp.rafiki.money/outgoing-payments/8c68d3cc-0a0f-4216-98b4-4fa44a6c88cf\","]
#[doc = "      \"metadata\": {"]
#[doc = "        \"description\": \"APlusVideo subscription\","]
#[doc = "        \"externalRef\": \"customer: 847458475\""]
#[doc = "      },"]
#[doc = "      \"receiveAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"2500\""]
#[doc = "      },"]
#[doc = "      \"receiver\": \"https://ilp.rafiki.money/aplusvideo/incoming-payments/45d495ad-b763-4882-88d7-aa14d261686e\","]
#[doc = "      \"sentAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"2500\""]
#[doc = "      },"]
#[doc = "      \"updatedAt\": \"2022-04-01T10:24:36.11Z\","]
#[doc = "      \"walletAddress\": \"https://ilp.rafiki.money/alice/\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"createdAt\": \"2022-03-12T23:20:50.52Z\","]
#[doc = "      \"debitAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"7126\""]
#[doc = "      },"]
#[doc = "      \"failed\": false,"]
#[doc = "      \"id\": \"https://ilp.rafiki.money/outgoing-payments/0cffa5a4-58fd-4cc8-8e01-7145c72bf07c\","]
#[doc = "      \"metadata\": {"]
#[doc = "        \"description\": \"Thank you for your purchase at ShoeShop!\","]
#[doc = "        \"externalRef\": \"INV2022-8943756\""]
#[doc = "      },"]
#[doc = "      \"receiver\": \"https://ilp.rafiki.money/shoeshop/2fe92c6f-ef0d-487c-8759-3784eae6bce9\","]
#[doc = "      \"sentAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"7026\""]
#[doc = "      },"]
#[doc = "      \"updatedAt\": \"2022-04-01T10:24:36.11Z\","]
#[doc = "      \"walletAddress\": \"https://ilp.rafiki.money/alice/\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"createdAt\","]
#[doc = "    \"debitAmount\","]
#[doc = "    \"id\","]
#[doc = "    \"receiveAmount\","]
#[doc = "    \"receiver\","]
#[doc = "    \"sentAmount\","]
#[doc = "    \"updatedAt\","]
#[doc = "    \"walletAddress\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"createdAt\": {"]
#[doc = "      \"description\": \"The date and time when the outgoing payment was created.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"debitAmount\": {"]
#[doc = "      \"description\": \"The total amount that should be deducted from the sender's account when this outgoing payment has been paid.\","]
#[doc = "      \"$ref\": \"#/$defs/amount\""]
#[doc = "    },"]
#[doc = "    \"failed\": {"]
#[doc = "      \"description\": \"Describes whether the payment failed to send its full amount.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The URL identifying the outgoing payment.\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Additional metadata associated with the outgoing payment. (Optional)\","]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"quoteId\": {"]
#[doc = "      \"description\": \"The URL of the quote defining this payment's amounts.\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"receiveAmount\": {"]
#[doc = "      \"description\": \"The total amount that should be received by the receiver when this outgoing payment has been paid.\","]
#[doc = "      \"$ref\": \"#/$defs/amount\""]
#[doc = "    },"]
#[doc = "    \"receiver\": {"]
#[doc = "      \"description\": \"The URL of the incoming payment that is being paid.\","]
#[doc = "      \"$ref\": \"#/$defs/receiver\""]
#[doc = "    },"]
#[doc = "    \"sentAmount\": {"]
#[doc = "      \"description\": \"The total amount that has been sent under this outgoing payment.\","]
#[doc = "      \"$ref\": \"#/$defs/amount\""]
#[doc = "    },"]
#[doc = "    \"updatedAt\": {"]
#[doc = "      \"description\": \"The date and time when the outgoing payment was updated.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"walletAddress\": {"]
#[doc = "      \"description\": \"The URL of the wallet address from which this payment is sent.\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OutgoingPayment {
    #[doc = "The date and time when the outgoing payment was created."]
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The total amount that should be deducted from the sender's account when this outgoing payment has been paid."]
    #[serde(rename = "debitAmount")]
    pub debit_amount: Amount,
    #[doc = "Describes whether the payment failed to send its full amount."]
    #[serde(default)]
    pub failed: bool,
    #[doc = "The URL identifying the outgoing payment."]
    pub id: ::std::string::String,
    #[doc = "Additional metadata associated with the outgoing payment. (Optional)"]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The URL of the quote defining this payment's amounts."]
    #[serde(
        rename = "quoteId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub quote_id: ::std::option::Option<::std::string::String>,
    #[doc = "The total amount that should be received by the receiver when this outgoing payment has been paid."]
    #[serde(rename = "receiveAmount")]
    pub receive_amount: Amount,
    #[doc = "The URL of the incoming payment that is being paid."]
    pub receiver: Receiver,
    #[doc = "The total amount that has been sent under this outgoing payment."]
    #[serde(rename = "sentAmount")]
    pub sent_amount: Amount,
    #[doc = "The date and time when the outgoing payment was updated."]
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The URL of the wallet address from which this payment is sent."]
    #[serde(rename = "walletAddress")]
    pub wallet_address: ::std::string::String,
}
impl ::std::convert::From<&OutgoingPayment> for OutgoingPayment {
    fn from(value: &OutgoingPayment) -> Self {
        value.clone()
    }
}
#[doc = ""]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"\","]
#[doc = "  \"examples\": ["]
#[doc = "    {"]
#[doc = "      \"endCursor\": \"315581f8-9967-45a0-9cd3-87b60b6d6414\","]
#[doc = "      \"hasNextPage\": true,"]
#[doc = "      \"hasPreviousPage\": true,"]
#[doc = "      \"startCursor\": \"241de237-f989-42be-926d-c0c1fca57708\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"hasNextPage\","]
#[doc = "    \"hasPreviousPage\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"endCursor\": {"]
#[doc = "      \"description\": \"Cursor corresponding to the last element in the result array.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"hasNextPage\": {"]
#[doc = "      \"description\": \"Describes whether the data set has further entries.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"hasPreviousPage\": {"]
#[doc = "      \"description\": \"Describes whether the data set has previous entries.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"startCursor\": {"]
#[doc = "      \"description\": \"Cursor corresponding to the first element in the result array.\","]
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
pub struct PageInfo {
    #[doc = "Cursor corresponding to the last element in the result array."]
    #[serde(
        rename = "endCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub end_cursor: ::std::option::Option<PageInfoEndCursor>,
    #[doc = "Describes whether the data set has further entries."]
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[doc = "Describes whether the data set has previous entries."]
    #[serde(rename = "hasPreviousPage")]
    pub has_previous_page: bool,
    #[doc = "Cursor corresponding to the first element in the result array."]
    #[serde(
        rename = "startCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub start_cursor: ::std::option::Option<PageInfoStartCursor>,
}
impl ::std::convert::From<&PageInfo> for PageInfo {
    fn from(value: &PageInfo) -> Self {
        value.clone()
    }
}
#[doc = "Cursor corresponding to the last element in the result array."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Cursor corresponding to the last element in the result array.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PageInfoEndCursor(::std::string::String);
impl ::std::ops::Deref for PageInfoEndCursor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PageInfoEndCursor> for ::std::string::String {
    fn from(value: PageInfoEndCursor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PageInfoEndCursor> for PageInfoEndCursor {
    fn from(value: &PageInfoEndCursor) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for PageInfoEndCursor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PageInfoEndCursor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PageInfoEndCursor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PageInfoEndCursor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PageInfoEndCursor {
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
#[doc = "Cursor corresponding to the first element in the result array."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Cursor corresponding to the first element in the result array.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PageInfoStartCursor(::std::string::String);
impl ::std::ops::Deref for PageInfoStartCursor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PageInfoStartCursor> for ::std::string::String {
    fn from(value: PageInfoStartCursor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PageInfoStartCursor> for PageInfoStartCursor {
    fn from(value: &PageInfoStartCursor) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for PageInfoStartCursor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PageInfoStartCursor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PageInfoStartCursor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PageInfoStartCursor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PageInfoStartCursor {
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
#[doc = "PaymentMethod"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ilp\""]
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
pub enum PaymentMethod {
    #[serde(rename = "ilp")]
    Ilp,
}
impl ::std::convert::From<&Self> for PaymentMethod {
    fn from(value: &PaymentMethod) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ilp => write!(f, "ilp"),
        }
    }
}
impl ::std::str::FromStr for PaymentMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ilp" => Ok(Self::Ilp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PaymentMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PaymentMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaymentMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An **incoming payment** resource with public details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Public Incoming Payment\","]
#[doc = "  \"description\": \"An **incoming payment** resource with public details.\","]
#[doc = "  \"examples\": ["]
#[doc = "    {"]
#[doc = "      \"receivedAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"authServer\": \"https://auth.rafiki.money\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"authServer\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"authServer\": {"]
#[doc = "      \"description\": \"The URL of the authorization server endpoint for getting grants and access tokens for this wallet address.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"receivedAmount\": {"]
#[doc = "      \"$ref\": \"#/$defs/amount\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unresolvedProperites\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PublicIncomingPayment {
    #[doc = "The URL of the authorization server endpoint for getting grants and access tokens for this wallet address."]
    #[serde(rename = "authServer")]
    pub auth_server: ::std::string::String,
    #[serde(
        rename = "receivedAmount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub received_amount: ::std::option::Option<Amount>,
}
impl ::std::convert::From<&PublicIncomingPayment> for PublicIncomingPayment {
    fn from(value: &PublicIncomingPayment) -> Self {
        value.clone()
    }
}
#[doc = "A **quote** resource represents the quoted amount details with which an Outgoing Payment may be created."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Quote\","]
#[doc = "  \"description\": \"A **quote** resource represents the quoted amount details with which an Outgoing Payment may be created.\","]
#[doc = "  \"examples\": ["]
#[doc = "    {"]
#[doc = "      \"createdAt\": \"2022-03-12T23:20:50.52Z\","]
#[doc = "      \"debitAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"2600\""]
#[doc = "      },"]
#[doc = "      \"expiresAt\": \"2022-04-12T23:20:50.52Z\","]
#[doc = "      \"id\": \"https://ilp.rafiki.money/quotes/ab03296b-0c8b-4776-b94e-7ee27d868d4d\","]
#[doc = "      \"method\": \"ilp\","]
#[doc = "      \"receiveAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"2500\""]
#[doc = "      },"]
#[doc = "      \"receiver\": \"https://ilp.rafiki.money/shoeshop/incoming-payments/2fe92c6f-ef0d-487c-8759-3784eae6bce9\","]
#[doc = "      \"sentAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"2500\""]
#[doc = "      },"]
#[doc = "      \"walletAddress\": \"https://ilp.rafiki.money/alice/\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"createdAt\": \"2022-03-12T23:20:50.52Z\","]
#[doc = "      \"debitAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"7126\""]
#[doc = "      },"]
#[doc = "      \"expiresAt\": \"2022-04-12T23:20:50.52Z\","]
#[doc = "      \"id\": \"https://ilp.rafiki.money/quotes/8c68d3cc-0a0f-4216-98b4-4fa44a6c88cf\","]
#[doc = "      \"method\": \"ilp\","]
#[doc = "      \"receiver\": \"https://ilp.rafiki.money/aplusvideo/incoming-payments/45d495ad-b763-4882-88d7-aa14d261686e\","]
#[doc = "      \"sentAmount\": {"]
#[doc = "        \"assetCode\": \"USD\","]
#[doc = "        \"assetScale\": 2,"]
#[doc = "        \"value\": \"7026\""]
#[doc = "      },"]
#[doc = "      \"walletAddress\": \"https://ilp.rafiki.money/alice/\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"createdAt\","]
#[doc = "    \"debitAmount\","]
#[doc = "    \"id\","]
#[doc = "    \"method\","]
#[doc = "    \"receiveAmount\","]
#[doc = "    \"receiver\","]
#[doc = "    \"walletAddress\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"createdAt\": {"]
#[doc = "      \"description\": \"The date and time when the quote was created.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"debitAmount\": {"]
#[doc = "      \"description\": \"The total amount that should be deducted from the sender's account when the corresponding outgoing payment has been paid. \","]
#[doc = "      \"$ref\": \"#/$defs/amount\""]
#[doc = "    },"]
#[doc = "    \"expiresAt\": {"]
#[doc = "      \"description\": \"The date and time when the calculated `debitAmount` is no longer valid.\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The URL identifying the quote.\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"$ref\": \"#/$defs/payment-method\""]
#[doc = "    },"]
#[doc = "    \"receiveAmount\": {"]
#[doc = "      \"description\": \"The total amount that should be received by the receiver when the corresponding outgoing payment has been paid.\","]
#[doc = "      \"$ref\": \"#/$defs/amount\""]
#[doc = "    },"]
#[doc = "    \"receiver\": {"]
#[doc = "      \"description\": \"The URL of the incoming payment that the quote is created for.\","]
#[doc = "      \"$ref\": \"#/$defs/receiver\""]
#[doc = "    },"]
#[doc = "    \"walletAddress\": {"]
#[doc = "      \"description\": \"The URL of the wallet address from which this quote's payment would be sent.\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Quote {
    #[doc = "The date and time when the quote was created."]
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The total amount that should be deducted from the sender's account when the corresponding outgoing payment has been paid. "]
    #[serde(rename = "debitAmount")]
    pub debit_amount: Amount,
    #[doc = "The date and time when the calculated `debitAmount` is no longer valid."]
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: ::std::option::Option<::std::string::String>,
    #[doc = "The URL identifying the quote."]
    pub id: ::std::string::String,
    pub method: PaymentMethod,
    #[doc = "The total amount that should be received by the receiver when the corresponding outgoing payment has been paid."]
    #[serde(rename = "receiveAmount")]
    pub receive_amount: Amount,
    #[doc = "The URL of the incoming payment that the quote is created for."]
    pub receiver: Receiver,
    #[doc = "The URL of the wallet address from which this quote's payment would be sent."]
    #[serde(rename = "walletAddress")]
    pub wallet_address: ::std::string::String,
}
impl ::std::convert::From<&Quote> for Quote {
    fn from(value: &Quote) -> Self {
        value.clone()
    }
}
#[doc = "The URL of the incoming payment that is being paid."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Receiver\","]
#[doc = "  \"description\": \"The URL of the incoming payment that is being paid.\","]
#[doc = "  \"examples\": ["]
#[doc = "    \"https://ilp.rafiki.money/incoming-payments/08394f02-7b7b-45e2-b645-51d04e7c330c\","]
#[doc = "    \"http://ilp.rafiki.money/incoming-payments/08394f02-7b7b-45e2-b645-51d04e7c330c\","]
#[doc = "    \"https://ilp.rafiki.money/incoming-payments/1\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"pattern\": \"^(https|http)://(.+)/incoming-payments/(.+)$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct Receiver(pub ::std::string::String);
impl ::std::ops::Deref for Receiver {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Receiver> for ::std::string::String {
    fn from(value: Receiver) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Receiver> for Receiver {
    fn from(value: &Receiver) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Receiver {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Receiver {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Receiver {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "URL of a wallet address hosted by a Rafiki instance."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Wallet Address\","]
#[doc = "  \"description\": \"URL of a wallet address hosted by a Rafiki instance.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct WalletAddressUri(pub ::std::string::String);
impl ::std::ops::Deref for WalletAddressUri {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WalletAddressUri> for ::std::string::String {
    fn from(value: WalletAddressUri) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WalletAddressUri> for WalletAddressUri {
    fn from(value: &WalletAddressUri) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for WalletAddressUri {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for WalletAddressUri {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for WalletAddressUri {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
