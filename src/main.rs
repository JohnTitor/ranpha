const SSID_DESC: &str = "network name (SSID)";
const PSK_DESC: &str = "network key (password)";
use crate::config::Config;
mod config;

fn main() {
    let _ = generateQrCode();
}

/// Build a schema for QR Code.
///
/// # Schema
///
/// ```text
/// WIFI:T:WPA;S:mynetwork;P:mypass;;
/// ^    ^     ^           ^
/// |    |     |           |
/// |    |     |           +-- WPA key
/// |    |     +-- encryption type
/// |    +-- ESSID
/// +-- code type
/// ```
fn buildSchema(config: Config) -> String {
    format!("WIFI:T:{};S:{};P:{};;", escapeSpecialCharacters(&config.encryption), escapeSpecialCharacters(&config.ssid), escapeSpecialCharacters(&config.key))
}

const SPECIAL_CHARACTERS: [&'static str; 5] = ["\\", ";", ",", "\"", ":"];

/// Escape some special characters.
///
/// Ref. https://github.com/zxing/zxing/wiki/Barcode-Contents#wi-fi-network-config-android-ios-11
fn escapeSpecialCharacters(string: &str) -> String {
    let mut string = string.clone();
    for c in SPECIAL_CHARACTERS {
        string.replace(c, &format!("\\{c}"));
    }

    string.to_string()
}

pub fn generateQrCode() {
    let config = Config::new("".to_string(), "".to_string(), "".to_string());
    let foo = buildSchema(config);
    qrcode_generator::to_png_to_file(foo, qrcode_generator::QrCodeEcc::High, 256, "./qr.png").unwrap();
}
