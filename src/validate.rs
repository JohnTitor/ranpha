use std::str::FromStr;

/// Special character list on QRCode.
///
/// Ref. https://github.com/zxing/zxing/wiki/Barcode-Contents#wi-fi-network-config-android-ios-11
const SPECIAL_CHARACTERS: [char; 5] = ['\\', ';', ',', '"', ':'];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Protocol {
    Wpa2,
    Wpa,
    Wep,
}

impl FromStr for Protocol {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "WPA2" => Ok(Self::Wpa2),
            "WPA" => Ok(Self::Wpa),
            "WEP" => Ok(Self::Wep),
            _ => Err("protocol is invalid, must be WPA2, WPA, or WEP"),
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wpa2 => write!(f, "WPA2"),
            Self::Wpa => write!(f, "WPA"),
            Self::Wep => write!(f, "WEP"),
        }
    }
}

/// Escape some special characters.
///
/// See also the link on `SPECIAL_CHARACTERS` const.
pub fn escape_special_characters(string: &str) -> String {
    let mut string = string.to_string();
    for c in SPECIAL_CHARACTERS {
        string = string.replace(c, &format!("\\{c}"));
    }

    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_special_characters() {
        let passed = r#"\Flow, the "flow" is in the air; floating forever:"#;
        let got = escape_special_characters(passed);
        let want = r#"\\Flow\, the \"flow\" is in the air\; floating forever\:"#;
        assert_eq!(want, got);

        let want = "Screams never he heard any more";
        let got = escape_special_characters(want);
        assert_eq!(want, got);
    }

    #[test]
    fn test_validate_encryption_protocol() -> Result<(), &'static str> {
        assert_eq!(Protocol::from_str("wPa2")?, Protocol::Wpa2);
        assert_eq!(Protocol::from_str("WPA2")?, Protocol::Wpa2);
        Ok(())
    }

    #[test]
    fn test_validate_encryption_protocol_should_panic() {
        assert!(Protocol::from_str("invalid protocol").is_err());
    }
}
