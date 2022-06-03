use std::error::Error;

use crate::config::Config;
use crate::validate::{escape_special_characters, validate_encryption_protocol};
use crate::Opts;

/// Generate QR code image.
/// Currently, supports PNG only.
pub fn generate_qr_code(opts: Opts, size: usize, path: &str) -> Result<(), Box<dyn Error>> {
    let config = Config::new(
        opts.ssid,
        opts.key,
        validate_encryption_protocol(opts.encryption_protocol),
    );
    let schema = build_schema(config);
    // FIXME: flexible QRcodeEcc
    qrcode_generator::to_png_to_file(schema, qrcode_generator::QrCodeEcc::High, size, path)?;

    Ok(())
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
fn build_schema(config: Config) -> String {
    let key = if config.encryption == "nopass" {
        ""
    } else {
        &config.key
    };
    format!(
        "WIFI:T:{};S:{};P:{};;",
        config.encryption,
        escape_special_characters(&config.ssid),
        escape_special_characters(key)
    )
}
