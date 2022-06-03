use qrcode_generator::QrCodeEcc;
use std::error::Error;

use crate::config::Config;
use crate::validate::{escape_special_characters, validate_encryption_protocol};
use crate::Opts;

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
fn build_schema(config: Config) -> String {
    let key = if config.encryption == "nopass" || config.key.is_empty() {
        "nopass"
    } else {
        &config.key
    };
    format!(
        "WIFI:T:{};S:{};P:{};;",
        config.encryption,
        escape_special_characters(&config.ssid),
        escape_special_characters(key),
    )
}

/// Generate QR code image.
/// Currently, supports PNG only.
pub fn generate_qr_code(opts: &Opts, size: usize, path: &str) -> Result<(), Box<dyn Error>> {
    let config = Config::new(
        opts.ssid.clone(),
        opts.key.clone(),
        validate_encryption_protocol(opts.encryption_protocol.clone()),
    );
    let schema = build_schema(config);

    if path.ends_with(".png") {
        qrcode_generator::to_png_to_file(schema, QrCodeEcc::High, size, path)?;
    } else if path.ends_with(".svg") {
        qrcode_generator::to_svg_to_file(schema, QrCodeEcc::High, size, None::<&str>, path)?;
    } else {
        unreachable!("image format must be PNG or SVG.")
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_schema() {
        let config = Config::new(
            "myssid".to_string(),
            "mykey".to_string(),
            "WPA2".to_string(),
        );
        let got = build_schema(config);
        let want = "WIFI:T:WPA2;S:myssid;P:mykey;;".to_string();
        assert_eq!(want, got);

        let config = Config::new("myssid".to_string(), "".to_string(), "WEP".to_string());
        let got = build_schema(config);
        let want = "WIFI:T:WEP;S:myssid;P:nopass;;".to_string();
        assert_eq!(want, got);

        let config = Config::new(
            "myssid".to_string(),
            "nopass".to_string(),
            "WPA".to_string(),
        );
        let got = build_schema(config);
        let want = "WIFI:T:WPA;S:myssid;P:nopass;;".to_string();
        assert_eq!(want, got);
    }
}
