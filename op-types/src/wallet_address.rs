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
#[doc = "A DID Document using JSON encoding"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"DID Document\","]
#[doc = "  \"description\": \"A DID Document using JSON encoding\","]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct DidDocument(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for DidDocument {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<DidDocument>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: DidDocument) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DidDocument> for DidDocument {
    fn from(value: &DidDocument) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for DidDocument
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "A JWK representation of an Ed25519 Public Key"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Ed25519 Public Key\","]
#[doc = "  \"description\": \"A JWK representation of an Ed25519 Public Key\","]
#[doc = "  \"examples\": ["]
#[doc = "    {"]
#[doc = "      \"crv\": \"Ed25519\","]
#[doc = "      \"kid\": \"key-1\","]
#[doc = "      \"kty\": \"OKP\","]
#[doc = "      \"use\": \"sig\","]
#[doc = "      \"x\": \"11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"crv\": \"Ed25519\","]
#[doc = "      \"kid\": \"2022-09-02\","]
#[doc = "      \"kty\": \"OKP\","]
#[doc = "      \"use\": \"sig\","]
#[doc = "      \"x\": \"oy0L_vTygNE4IogRyn_F5GmHXdqYVjIXkWs2jky7zsI\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"alg\","]
#[doc = "    \"crv\","]
#[doc = "    \"kid\","]
#[doc = "    \"kty\","]
#[doc = "    \"x\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alg\": {"]
#[doc = "      \"description\": \"The cryptographic algorithm family used with the key. The only allowed value is `EdDSA`. \","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"EdDSA\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"crv\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Ed25519\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"kid\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"kty\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"OKP\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"use\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"sig\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"x\": {"]
#[doc = "      \"description\": \"The base64 url-encoded public key.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[a-zA-Z0-9-_]+$\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonWebKey {
    #[doc = "The cryptographic algorithm family used with the key. The only allowed value is `EdDSA`. "]
    pub alg: JsonWebKeyAlg,
    pub crv: JsonWebKeyCrv,
    pub kid: ::std::string::String,
    pub kty: JsonWebKeyKty,
    #[serde(
        rename = "use",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_: ::std::option::Option<JsonWebKeyUse>,
    #[doc = "The base64 url-encoded public key."]
    pub x: JsonWebKeyX,
}
impl ::std::convert::From<&JsonWebKey> for JsonWebKey {
    fn from(value: &JsonWebKey) -> Self {
        value.clone()
    }
}
#[doc = "The cryptographic algorithm family used with the key. The only allowed value is `EdDSA`. "]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The cryptographic algorithm family used with the key. The only allowed value is `EdDSA`. \","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"EdDSA\""]
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
pub enum JsonWebKeyAlg {
    #[serde(rename = "EdDSA")]
    EdDsa,
}
impl ::std::convert::From<&Self> for JsonWebKeyAlg {
    fn from(value: &JsonWebKeyAlg) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for JsonWebKeyAlg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::EdDsa => write!(f, "EdDSA"),
        }
    }
}
impl ::std::str::FromStr for JsonWebKeyAlg {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EdDSA" => Ok(Self::EdDsa),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonWebKeyAlg {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonWebKeyAlg {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonWebKeyAlg {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "JsonWebKeyCrv"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Ed25519\""]
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
pub enum JsonWebKeyCrv {
    Ed25519,
}
impl ::std::convert::From<&Self> for JsonWebKeyCrv {
    fn from(value: &JsonWebKeyCrv) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for JsonWebKeyCrv {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ed25519 => write!(f, "Ed25519"),
        }
    }
}
impl ::std::str::FromStr for JsonWebKeyCrv {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Ed25519" => Ok(Self::Ed25519),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonWebKeyCrv {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonWebKeyCrv {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonWebKeyCrv {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "JsonWebKeyKty"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"OKP\""]
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
pub enum JsonWebKeyKty {
    #[serde(rename = "OKP")]
    Okp,
}
impl ::std::convert::From<&Self> for JsonWebKeyKty {
    fn from(value: &JsonWebKeyKty) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for JsonWebKeyKty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Okp => write!(f, "OKP"),
        }
    }
}
impl ::std::str::FromStr for JsonWebKeyKty {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "OKP" => Ok(Self::Okp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonWebKeyKty {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonWebKeyKty {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonWebKeyKty {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A JSON Web Key Set document according to [rfc7517](https://datatracker.ietf.org/doc/html/rfc7517) listing the keys associated with this wallet address. These keys are used to sign requests made by this wallet address."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"JSON Web Key Set document\","]
#[doc = "  \"description\": \"A JSON Web Key Set document according to [rfc7517](https://datatracker.ietf.org/doc/html/rfc7517) listing the keys associated with this wallet address. These keys are used to sign requests made by this wallet address.\","]
#[doc = "  \"examples\": ["]
#[doc = "    {"]
#[doc = "      \"keys\": ["]
#[doc = "        {"]
#[doc = "          \"alg\": \"EdDSA\","]
#[doc = "          \"crv\": \"Ed25519\","]
#[doc = "          \"kid\": \"key-1\","]
#[doc = "          \"kty\": \"OKP\","]
#[doc = "          \"use\": \"sig\","]
#[doc = "          \"x\": \"11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"keys\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"keys\": {"]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/json-web-key\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonWebKeySet {
    pub keys: ::std::vec::Vec<JsonWebKey>,
}
impl ::std::convert::From<&JsonWebKeySet> for JsonWebKeySet {
    fn from(value: &JsonWebKeySet) -> Self {
        value.clone()
    }
}
#[doc = "JsonWebKeyUse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"sig\""]
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
pub enum JsonWebKeyUse {
    #[serde(rename = "sig")]
    Sig,
}
impl ::std::convert::From<&Self> for JsonWebKeyUse {
    fn from(value: &JsonWebKeyUse) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for JsonWebKeyUse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sig => write!(f, "sig"),
        }
    }
}
impl ::std::str::FromStr for JsonWebKeyUse {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "sig" => Ok(Self::Sig),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonWebKeyUse {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonWebKeyUse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonWebKeyUse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The base64 url-encoded public key."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The base64 url-encoded public key.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z0-9-_]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JsonWebKeyX(::std::string::String);
impl ::std::ops::Deref for JsonWebKeyX {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JsonWebKeyX> for ::std::string::String {
    fn from(value: JsonWebKeyX) -> Self {
        value.0
    }
}
impl ::std::convert::From<&JsonWebKeyX> for JsonWebKeyX {
    fn from(value: &JsonWebKeyX) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for JsonWebKeyX {
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
impl ::std::convert::TryFrom<&str> for JsonWebKeyX {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonWebKeyX {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonWebKeyX {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JsonWebKeyX {
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
#[doc = "A **wallet address** resource is the root of the API and contains the public details of the financial account represented by the Wallet Address that is also the service endpoint URL."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Wallet Address\","]
#[doc = "  \"description\": \"A **wallet address** resource is the root of the API and contains the public details of the financial account represented by the Wallet Address that is also the service endpoint URL.\","]
#[doc = "  \"examples\": ["]
#[doc = "    {"]
#[doc = "      \"assetCode\": \"USD\","]
#[doc = "      \"assetScale\": 2,"]
#[doc = "      \"authServer\": \"https://rafiki.money/auth\","]
#[doc = "      \"id\": \"https://rafiki.money/alice\","]
#[doc = "      \"publicName\": \"Alice\","]
#[doc = "      \"resourceServer\": \"https://rafiki.money/op\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"assetCode\","]
#[doc = "    \"assetScale\","]
#[doc = "    \"authServer\","]
#[doc = "    \"id\","]
#[doc = "    \"resourceServer\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"assetCode\": {"]
#[doc = "      \"$ref\": \"#/$defs/assetCode\""]
#[doc = "    },"]
#[doc = "    \"assetScale\": {"]
#[doc = "      \"$ref\": \"#/$defs/assetScale\""]
#[doc = "    },"]
#[doc = "    \"authServer\": {"]
#[doc = "      \"description\": \"The URL of the authorization server endpoint for getting grants and access tokens for this wallet address.\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The URL identifying the wallet address.\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"publicName\": {"]
#[doc = "      \"description\": \"A public name for the account. This should be set by the account holder with their provider to provide a hint to counterparties as to the identity of the account holder.\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"resourceServer\": {"]
#[doc = "      \"description\": \"The URL of the resource server endpoint for performing Open Payments with this wallet address.\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WalletAddress {
    #[serde(rename = "assetCode")]
    pub asset_code: AssetCode,
    #[serde(rename = "assetScale")]
    pub asset_scale: AssetScale,
    #[doc = "The URL of the authorization server endpoint for getting grants and access tokens for this wallet address."]
    #[serde(rename = "authServer")]
    pub auth_server: ::std::string::String,
    #[doc = "The URL identifying the wallet address."]
    pub id: ::std::string::String,
    #[doc = "A public name for the account. This should be set by the account holder with their provider to provide a hint to counterparties as to the identity of the account holder."]
    #[serde(
        rename = "publicName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_name: ::std::option::Option<::std::string::String>,
    #[doc = "The URL of the resource server endpoint for performing Open Payments with this wallet address."]
    #[serde(rename = "resourceServer")]
    pub resource_server: ::std::string::String,
}
impl ::std::convert::From<&WalletAddress> for WalletAddress {
    fn from(value: &WalletAddress) -> Self {
        value.clone()
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
