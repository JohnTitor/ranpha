pub fn validate_encryption_protocol(protocol: String) -> String {
    let protocol = protocol.to_uppercase();
    match protocol.as_str() {
        "WPA2" | "WPA" | "WEP" => protocol,
        _ => "nopass".to_string(),
    }
}

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
