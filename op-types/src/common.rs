use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fmt;
use std::str::FromStr;

// Matches R[n]/P... format where [n] is optional number
static INTERVAL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^R\d*/P|^R/P").unwrap());

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub value: u64,
    pub asset_code: String,
    pub asset_scale: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Receiver(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WalletAddressUri(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ISO8601Format(String);

impl FromStr for ISO8601Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if INTERVAL_REGEX.is_match(s) {
            Ok(Self(s.to_string()))
        } else {
            Err("Invalid ISO8601 repeating interval format. Must start with R[n]/P".to_string())
        }
    }
}

impl fmt::Display for ISO8601Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Interval(#[serde_as(as = "serde_with::DisplayFromStr")] pub ISO8601Format);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_intervals() {
        let valid = vec![
            "R/P1D",   // Repeat forever, daily
            "R3/P1M",  // Repeat 3 times, monthly
            "R5/PT1H", // Repeat 5 times, hourly
        ];

        for case in valid {
            let interval: Interval = serde_json::from_value(json!(case)).unwrap();
            assert_eq!(interval.0.0, case);
        }
    }

    #[test]
    fn test_invalid_intervals() {
        let invalid = vec![
            "P1D",     // Missing R prefix
            "R1D",     // Missing /P
            "invalid", // Invalid format
            "",        // Empty string
        ];

        for case in invalid {
            assert!(serde_json::from_value::<Interval>(json!(case)).is_err());
        }
    }
}
