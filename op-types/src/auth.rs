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
#[doc = "A description of the rights associated with this access token."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A description of the rights associated with this access token.\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/access-item\""]
#[doc = "  },"]
#[doc = "  \"maxItems\": 3,"]
#[doc = "  \"uniqueItems\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Access(pub Vec<AccessItem>);
impl ::std::ops::Deref for Access {
    type Target = Vec<AccessItem>;
    fn deref(&self) -> &Vec<AccessItem> {
        &self.0
    }
}
impl ::std::convert::From<Access> for Vec<AccessItem> {
    fn from(value: Access) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Access> for Access {
    fn from(value: &Access) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Vec<AccessItem>> for Access {
    fn from(value: Vec<AccessItem>) -> Self {
        Self(value)
    }
}
#[doc = "AccessIncoming"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"access-incoming\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"actions\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"actions\": {"]
#[doc = "      \"description\": \"The types of actions the client instance will take at the RS as an array of strings.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"create\","]
#[doc = "          \"complete\","]
#[doc = "          \"read\","]
#[doc = "          \"read-all\","]
#[doc = "          \"list\","]
#[doc = "          \"list-all\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"identifier\": {"]
#[doc = "      \"description\": \"A string identifier indicating a specific resource at the RS.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The type of resource request as a string.  This field defines which other fields are allowed in the request object.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"incoming-payment\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AccessIncoming {
    #[doc = "The types of actions the client instance will take at the RS as an array of strings."]
    pub actions: Vec<AccessIncomingActionsItem>,
    #[doc = "A string identifier indicating a specific resource at the RS."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub identifier: ::std::option::Option<::std::string::String>,
    #[doc = "The type of resource request as a string.  This field defines which other fields are allowed in the request object."]
    #[serde(rename = "type")]
    pub type_: AccessIncomingType,
}
impl ::std::convert::From<&AccessIncoming> for AccessIncoming {
    fn from(value: &AccessIncoming) -> Self {
        value.clone()
    }
}
#[doc = "AccessIncomingActionsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"create\","]
#[doc = "    \"complete\","]
#[doc = "    \"read\","]
#[doc = "    \"read-all\","]
#[doc = "    \"list\","]
#[doc = "    \"list-all\""]
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
pub enum AccessIncomingActionsItem {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "read-all")]
    ReadAll,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "list-all")]
    ListAll,
}
impl ::std::convert::From<&Self> for AccessIncomingActionsItem {
    fn from(value: &AccessIncomingActionsItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AccessIncomingActionsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Create => write!(f, "create"),
            Self::Complete => write!(f, "complete"),
            Self::Read => write!(f, "read"),
            Self::ReadAll => write!(f, "read-all"),
            Self::List => write!(f, "list"),
            Self::ListAll => write!(f, "list-all"),
        }
    }
}
impl ::std::str::FromStr for AccessIncomingActionsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "create" => Ok(Self::Create),
            "complete" => Ok(Self::Complete),
            "read" => Ok(Self::Read),
            "read-all" => Ok(Self::ReadAll),
            "list" => Ok(Self::List),
            "list-all" => Ok(Self::ListAll),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccessIncomingActionsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccessIncomingActionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccessIncomingActionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of resource request as a string.  This field defines which other fields are allowed in the request object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The type of resource request as a string.  This field defines which other fields are allowed in the request object.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"incoming-payment\""]
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
pub enum AccessIncomingType {
    #[serde(rename = "incoming-payment")]
    IncomingPayment,
}
impl ::std::convert::From<&Self> for AccessIncomingType {
    fn from(value: &AccessIncomingType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AccessIncomingType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::IncomingPayment => write!(f, "incoming-payment"),
        }
    }
}
impl ::std::str::FromStr for AccessIncomingType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "incoming-payment" => Ok(Self::IncomingPayment),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccessIncomingType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccessIncomingType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccessIncomingType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The access associated with the access token is described using objects that each contain multiple dimensions of access."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The access associated with the access token is described using objects that each contain multiple dimensions of access.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/access-incoming\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/access-outgoing\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/access-quote\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AccessItem {
    Incoming(AccessIncoming),
    Outgoing(AccessOutgoing),
    Quote(AccessQuote),
}
impl ::std::convert::From<&Self> for AccessItem {
    fn from(value: &AccessItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<AccessIncoming> for AccessItem {
    fn from(value: AccessIncoming) -> Self {
        Self::Incoming(value)
    }
}
impl ::std::convert::From<AccessOutgoing> for AccessItem {
    fn from(value: AccessOutgoing) -> Self {
        Self::Outgoing(value)
    }
}
impl ::std::convert::From<AccessQuote> for AccessItem {
    fn from(value: AccessQuote) -> Self {
        Self::Quote(value)
    }
}
#[doc = "AccessOutgoing"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"access-outgoing\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"actions\","]
#[doc = "    \"identifier\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"actions\": {"]
#[doc = "      \"description\": \"The types of actions the client instance will take at the RS as an array of strings.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"create\","]
#[doc = "          \"read\","]
#[doc = "          \"read-all\","]
#[doc = "          \"list\","]
#[doc = "          \"list-all\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"identifier\": {"]
#[doc = "      \"description\": \"A string identifier indicating a specific resource at the RS.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"limits\": {"]
#[doc = "      \"$ref\": \"#/$defs/limits-outgoing\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The type of resource request as a string.  This field defines which other fields are allowed in the request object.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"outgoing-payment\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AccessOutgoing {
    #[doc = "The types of actions the client instance will take at the RS as an array of strings."]
    pub actions: Vec<AccessOutgoingActionsItem>,
    #[doc = "A string identifier indicating a specific resource at the RS."]
    pub identifier: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub limits: ::std::option::Option<LimitsOutgoing>,
    #[doc = "The type of resource request as a string.  This field defines which other fields are allowed in the request object."]
    #[serde(rename = "type")]
    pub type_: AccessOutgoingType,
}
impl ::std::convert::From<&AccessOutgoing> for AccessOutgoing {
    fn from(value: &AccessOutgoing) -> Self {
        value.clone()
    }
}
#[doc = "AccessOutgoingActionsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"create\","]
#[doc = "    \"read\","]
#[doc = "    \"read-all\","]
#[doc = "    \"list\","]
#[doc = "    \"list-all\""]
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
pub enum AccessOutgoingActionsItem {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "read-all")]
    ReadAll,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "list-all")]
    ListAll,
}
impl ::std::convert::From<&Self> for AccessOutgoingActionsItem {
    fn from(value: &AccessOutgoingActionsItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AccessOutgoingActionsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Create => write!(f, "create"),
            Self::Read => write!(f, "read"),
            Self::ReadAll => write!(f, "read-all"),
            Self::List => write!(f, "list"),
            Self::ListAll => write!(f, "list-all"),
        }
    }
}
impl ::std::str::FromStr for AccessOutgoingActionsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "create" => Ok(Self::Create),
            "read" => Ok(Self::Read),
            "read-all" => Ok(Self::ReadAll),
            "list" => Ok(Self::List),
            "list-all" => Ok(Self::ListAll),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccessOutgoingActionsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccessOutgoingActionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccessOutgoingActionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of resource request as a string.  This field defines which other fields are allowed in the request object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The type of resource request as a string.  This field defines which other fields are allowed in the request object.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"outgoing-payment\""]
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
pub enum AccessOutgoingType {
    #[serde(rename = "outgoing-payment")]
    OutgoingPayment,
}
impl ::std::convert::From<&Self> for AccessOutgoingType {
    fn from(value: &AccessOutgoingType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AccessOutgoingType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OutgoingPayment => write!(f, "outgoing-payment"),
        }
    }
}
impl ::std::str::FromStr for AccessOutgoingType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "outgoing-payment" => Ok(Self::OutgoingPayment),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccessOutgoingType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccessOutgoingType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccessOutgoingType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "AccessQuote"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"access-quote\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"actions\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"actions\": {"]
#[doc = "      \"description\": \"The types of actions the client instance will take at the RS as an array of strings.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"create\","]
#[doc = "          \"read\","]
#[doc = "          \"read-all\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The type of resource request as a string.  This field defines which other fields are allowed in the request object.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"quote\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AccessQuote {
    #[doc = "The types of actions the client instance will take at the RS as an array of strings."]
    pub actions: Vec<AccessQuoteActionsItem>,
    #[doc = "The type of resource request as a string.  This field defines which other fields are allowed in the request object."]
    #[serde(rename = "type")]
    pub type_: AccessQuoteType,
}
impl ::std::convert::From<&AccessQuote> for AccessQuote {
    fn from(value: &AccessQuote) -> Self {
        value.clone()
    }
}
#[doc = "AccessQuoteActionsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"create\","]
#[doc = "    \"read\","]
#[doc = "    \"read-all\""]
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
pub enum AccessQuoteActionsItem {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "read-all")]
    ReadAll,
}
impl ::std::convert::From<&Self> for AccessQuoteActionsItem {
    fn from(value: &AccessQuoteActionsItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AccessQuoteActionsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Create => write!(f, "create"),
            Self::Read => write!(f, "read"),
            Self::ReadAll => write!(f, "read-all"),
        }
    }
}
impl ::std::str::FromStr for AccessQuoteActionsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "create" => Ok(Self::Create),
            "read" => Ok(Self::Read),
            "read-all" => Ok(Self::ReadAll),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccessQuoteActionsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccessQuoteActionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccessQuoteActionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of resource request as a string.  This field defines which other fields are allowed in the request object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The type of resource request as a string.  This field defines which other fields are allowed in the request object.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"quote\""]
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
pub enum AccessQuoteType {
    #[serde(rename = "quote")]
    Quote,
}
impl ::std::convert::From<&Self> for AccessQuoteType {
    fn from(value: &AccessQuoteType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AccessQuoteType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Quote => write!(f, "quote"),
        }
    }
}
impl ::std::str::FromStr for AccessQuoteType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "quote" => Ok(Self::Quote),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccessQuoteType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccessQuoteType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccessQuoteType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A single access token or set of access tokens that the client instance can use to call the RS on behalf of the RO."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"access_token\","]
#[doc = "  \"description\": \"A single access token or set of access tokens that the client instance can use to call the RS on behalf of the RO.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"access\","]
#[doc = "    \"manage\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"access\": {"]
#[doc = "      \"$ref\": \"#/$defs/access\""]
#[doc = "    },"]
#[doc = "    \"expires_in\": {"]
#[doc = "      \"description\": \"The number of seconds in which the access will expire.  The client instance MUST NOT use the access token past this time.  An RS MUST NOT accept an access token past this time.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"manage\": {"]
#[doc = "      \"description\": \"The management URI for this access token. This URI MUST NOT include the access token value and SHOULD be different for each access token issued in a request.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The value of the access token as a string.  The value is opaque to the client instance.  The value SHOULD be limited to ASCII characters to facilitate transmission over HTTP headers within other protocols without requiring additional encoding.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AccessToken {
    pub access: Access,
    #[doc = "The number of seconds in which the access will expire.  The client instance MUST NOT use the access token past this time.  An RS MUST NOT accept an access token past this time."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_in: ::std::option::Option<i64>,
    #[doc = "The management URI for this access token. This URI MUST NOT include the access token value and SHOULD be different for each access token issued in a request."]
    pub manage: ::std::string::String,
    #[doc = "The value of the access token as a string.  The value is opaque to the client instance.  The value SHOULD be limited to ASCII characters to facilitate transmission over HTTP headers within other protocols without requiring additional encoding."]
    pub value: ::std::string::String,
}
impl ::std::convert::From<&AccessToken> for AccessToken {
    fn from(value: &AccessToken) -> Self {
        value.clone()
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
#[doc = "Wallet address of the client instance that is making this request.\n\nWhen sending a non-continuation request to the AS, the client instance MUST identify itself by including the client field of the request and by signing the request.\n\nA JSON Web Key Set document, including the public key that the client instance will use to protect this request and any continuation requests at the AS and any user-facing information about the client instance used in interactions, MUST be available at the wallet address + `/jwks.json` url.\n\nIf sending a grant initiation request that requires RO interaction, the wallet address MUST serve necessary client display information."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"client\","]
#[doc = "  \"description\": \"Wallet address of the client instance that is making this request.\\n\\nWhen sending a non-continuation request to the AS, the client instance MUST identify itself by including the client field of the request and by signing the request.\\n\\nA JSON Web Key Set document, including the public key that the client instance will use to protect this request and any continuation requests at the AS and any user-facing information about the client instance used in interactions, MUST be available at the wallet address + `/jwks.json` url.\\n\\nIf sending a grant initiation request that requires RO interaction, the wallet address MUST serve necessary client display information.\","]
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
pub struct Client(pub ::std::string::String);
impl ::std::ops::Deref for Client {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Client> for ::std::string::String {
    fn from(value: Client) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Client> for Client {
    fn from(value: &Client) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Client {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Client {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Client {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "If the AS determines that the request can be continued with additional requests, it responds with the continue field."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"continue\","]
#[doc = "  \"description\": \"If the AS determines that the request can be continued with additional requests, it responds with the continue field.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"access_token\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"access_token\": {"]
#[doc = "      \"description\": \"A unique access token for continuing the request, called the \\\"continuation access token\\\".\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI at which the client instance can make continuation requests.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"wait\": {"]
#[doc = "      \"description\": \"The amount of time in integer seconds the client instance MUST wait after receiving this request continuation response and calling the continuation URI.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Continue {
    pub access_token: ContinueAccessToken,
    #[doc = "The URI at which the client instance can make continuation requests."]
    pub uri: ::std::string::String,
    #[doc = "The amount of time in integer seconds the client instance MUST wait after receiving this request continuation response and calling the continuation URI."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wait: ::std::option::Option<i64>,
}
impl ::std::convert::From<&Continue> for Continue {
    fn from(value: &Continue) -> Self {
        value.clone()
    }
}
#[doc = "A unique access token for continuing the request, called the \"continuation access token\"."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A unique access token for continuing the request, called the \\\"continuation access token\\\".\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ContinueAccessToken {
    pub value: ::std::string::String,
}
impl ::std::convert::From<&ContinueAccessToken> for ContinueAccessToken {
    fn from(value: &ContinueAccessToken) -> Self {
        value.clone()
    }
}
#[doc = "The client instance declares the parameters for interaction methods that it can support using the interact field."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"interact\","]
#[doc = "  \"description\": \"The client instance declares the parameters for interaction methods that it can support using the interact field.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"start\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"finish\": {"]
#[doc = "      \"description\": \"Indicates how the client instance can receive an indication that interaction has finished at the AS.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"nonce\","]
#[doc = "        \"uri\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"description\": \"The callback method that the AS will use to contact the client instance.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"redirect\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"nonce\": {"]
#[doc = "          \"description\": \"Unique value to be used in the calculation of the \\\"hash\\\" query parameter sent to the callback URI, must be sufficiently random to be unguessable by an attacker.  MUST be generated by the client instance as a unique value for this request.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"uri\": {"]
#[doc = "          \"description\": \"Indicates the URI that the AS will either send the RO to after interaction or send an HTTP POST request.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"description\": \"Indicates how the client instance can start an interaction.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"redirect\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InteractRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub finish: ::std::option::Option<InteractRequestFinish>,
    #[doc = "Indicates how the client instance can start an interaction."]
    pub start: ::std::vec::Vec<InteractRequestStartItem>,
}
impl ::std::convert::From<&InteractRequest> for InteractRequest {
    fn from(value: &InteractRequest) -> Self {
        value.clone()
    }
}
#[doc = "Indicates how the client instance can receive an indication that interaction has finished at the AS."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Indicates how the client instance can receive an indication that interaction has finished at the AS.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"nonce\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"description\": \"The callback method that the AS will use to contact the client instance.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"redirect\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nonce\": {"]
#[doc = "      \"description\": \"Unique value to be used in the calculation of the \\\"hash\\\" query parameter sent to the callback URI, must be sufficiently random to be unguessable by an attacker.  MUST be generated by the client instance as a unique value for this request.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"Indicates the URI that the AS will either send the RO to after interaction or send an HTTP POST request.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InteractRequestFinish {
    #[doc = "The callback method that the AS will use to contact the client instance."]
    pub method: InteractRequestFinishMethod,
    #[doc = "Unique value to be used in the calculation of the \"hash\" query parameter sent to the callback URI, must be sufficiently random to be unguessable by an attacker.  MUST be generated by the client instance as a unique value for this request."]
    pub nonce: ::std::string::String,
    #[doc = "Indicates the URI that the AS will either send the RO to after interaction or send an HTTP POST request."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&InteractRequestFinish> for InteractRequestFinish {
    fn from(value: &InteractRequestFinish) -> Self {
        value.clone()
    }
}
#[doc = "The callback method that the AS will use to contact the client instance."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The callback method that the AS will use to contact the client instance.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"redirect\""]
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
pub enum InteractRequestFinishMethod {
    #[serde(rename = "redirect")]
    Redirect,
}
impl ::std::convert::From<&Self> for InteractRequestFinishMethod {
    fn from(value: &InteractRequestFinishMethod) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for InteractRequestFinishMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Redirect => write!(f, "redirect"),
        }
    }
}
impl ::std::str::FromStr for InteractRequestFinishMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "redirect" => Ok(Self::Redirect),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InteractRequestFinishMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InteractRequestFinishMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InteractRequestFinishMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "InteractRequestStartItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"redirect\""]
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
pub enum InteractRequestStartItem {
    #[serde(rename = "redirect")]
    Redirect,
}
impl ::std::convert::From<&Self> for InteractRequestStartItem {
    fn from(value: &InteractRequestStartItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for InteractRequestStartItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Redirect => write!(f, "redirect"),
        }
    }
}
impl ::std::str::FromStr for InteractRequestStartItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "redirect" => Ok(Self::Redirect),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InteractRequestStartItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for InteractRequestStartItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InteractRequestStartItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "InteractResponse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"interact-response\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"finish\","]
#[doc = "    \"redirect\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"finish\": {"]
#[doc = "      \"description\": \"Unique key to secure the callback.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"redirect\": {"]
#[doc = "      \"description\": \"The URI to direct the end user to.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InteractResponse {
    #[doc = "Unique key to secure the callback."]
    pub finish: ::std::string::String,
    #[doc = "The URI to direct the end user to."]
    pub redirect: ::std::string::String,
}
impl ::std::convert::From<&InteractResponse> for InteractResponse {
    fn from(value: &InteractResponse) -> Self {
        value.clone()
    }
}
#[doc = "[ISO8601 repeating interval](https://en.wikipedia.org/wiki/ISO_8601#Repeating_intervals)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Interval\","]
#[doc = "  \"description\": \"[ISO8601 repeating interval](https://en.wikipedia.org/wiki/ISO_8601#Repeating_intervals)\","]
#[doc = "  \"examples\": ["]
#[doc = "    \"R11/2022-08-24T14:15:22Z/P1M\","]
#[doc = "    \"R/2017-03-01T13:00:00Z/2018-05-11T15:30:00Z\","]
#[doc = "    \"R-1/P1Y2M10DT2H30M/2022-05-11T15:30:00Z\""]
#[doc = "  ],"]
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
pub struct Interval(pub ::std::string::String);
impl ::std::ops::Deref for Interval {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Interval> for ::std::string::String {
    fn from(value: Interval) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Interval> for Interval {
    fn from(value: &Interval) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Interval {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Interval {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Interval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "Open Payments specific property that defines the limits under which outgoing payments can be created."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"limits-outgoing\","]
#[doc = "  \"description\": \"Open Payments specific property that defines the limits under which outgoing payments can be created.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"required\": ["]
#[doc = "          \"interval\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"debitAmount\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"receiveAmount\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"debitAmount\": {"]
#[doc = "      \"description\": \"All amounts are maxima, i.e. multiple payments can be created under a grant as long as the total amounts of these payments do not exceed the maximum amount per interval as specified in the grant.\","]
#[doc = "      \"$ref\": \"#/$defs/amount\""]
#[doc = "    },"]
#[doc = "    \"interval\": {"]
#[doc = "      \"$ref\": \"#/$defs/interval\""]
#[doc = "    },"]
#[doc = "    \"receiveAmount\": {"]
#[doc = "      \"description\": \"All amounts are maxima, i.e. multiple payments can be created under a grant as long as the total amounts of these payments do not exceed the maximum amount per interval as specified in the grant.\","]
#[doc = "      \"$ref\": \"#/$defs/amount\""]
#[doc = "    },"]
#[doc = "    \"receiver\": {"]
#[doc = "      \"$ref\": \"#/$defs/receiver\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum LimitsOutgoing {
    Variant0 {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        receiver: ::std::option::Option<Receiver>,
    },
    Variant1 {
        #[doc = "All amounts are maxima, i.e. multiple payments can be created under a grant as long as the total amounts of these payments do not exceed the maximum amount per interval as specified in the grant."]
        #[serde(rename = "debitAmount")]
        debit_amount: Amount,
        interval: Interval,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        receiver: ::std::option::Option<Receiver>,
    },
    Variant2 {
        interval: Interval,
        #[doc = "All amounts are maxima, i.e. multiple payments can be created under a grant as long as the total amounts of these payments do not exceed the maximum amount per interval as specified in the grant."]
        #[serde(rename = "receiveAmount")]
        receive_amount: Amount,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        receiver: ::std::option::Option<Receiver>,
    },
}
impl ::std::convert::From<&Self> for LimitsOutgoing {
    fn from(value: &LimitsOutgoing) -> Self {
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
