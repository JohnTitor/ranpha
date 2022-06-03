/// Special character list on QRCode.
///
/// Ref. https://github.com/zxing/zxing/wiki/Barcode-Contents#wi-fi-network-config-android-ios-11
const SPECIAL_CHARACTERS: [char; 5] = ['\\', ';', ',', '"', ':'];

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

/// Validate encryption protocol.
/// Currently, only accepts WPA2, WPA, or WEP.
pub fn validate_encryption_protocol(protocol: String) -> String {
    let protocol = protocol.to_uppercase();
    match protocol.as_str() {
        "WPA2" | "WPA" | "WEP" => protocol,
        _ => unreachable!("protocol is invalid, must be WPA2, WPA, or WEP"),
    }
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
    fn test_validate_encryption_protocol() {
        let passed = "wPa2".to_string();
        let got = validate_encryption_protocol(passed);
        let want = "WPA2";
        assert_eq!(want, got);
    }

    #[test]
    #[should_panic]
    fn test_validate_encryption_protocol_should_panic() {
        let _ = validate_encryption_protocol("invalid protocol".to_string());
    }
}
