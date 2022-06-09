use qrcode_generator::QrCodeEcc;
use std::error::Error;

use crate::config::Config;
use crate::validate::{escape_special_characters, ImageFormat};
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
    format!(
        "WIFI:T:{};S:{};P:{};;",
        config.encryption,
        escape_special_characters(&config.ssid),
        escape_special_characters(config.key.as_ref().map_or("nopass", |key| key)),
    )
}

/// Generate QR code image.
/// Currently, supports PNG only.
pub fn generate_qr_code(opts: &Opts, size: usize, path: &str) -> Result<(), Box<dyn Error>> {
    let config = Config::new(
        opts.ssid.clone(),
        opts.key.clone(),
        opts.encryption_protocol,
    );
    let schema = build_schema(config);

    match opts.image_format {
        ImageFormat::Png => qrcode_generator::to_png_to_file(schema, QrCodeEcc::High, size, path)?,
        ImageFormat::Svg => {
            qrcode_generator::to_svg_to_file(schema, QrCodeEcc::High, size, None::<&str>, path)?
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::Protocol;

    #[test]
    fn test_build_schema() {
        let config = Config::new(
            "myssid".to_string(),
            Some("mykey".to_string()),
            Protocol::Wpa2,
        );
        let got = build_schema(config);
        let want = "WIFI:T:WPA2;S:myssid;P:mykey;;".to_string();
        assert_eq!(want, got);

        let config = Config::new("myssid".to_string(), None, Protocol::Wep);
        let got = build_schema(config);
        let want = "WIFI:T:WEP;S:myssid;P:nopass;;".to_string();
        assert_eq!(want, got);

        let config = Config::new("myssid".to_string(), None, Protocol::Wpa);
        let got = build_schema(config);
        let want = "WIFI:T:WPA;S:myssid;P:nopass;;".to_string();
        assert_eq!(want, got);
    }
}
