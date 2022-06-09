use crate::validate::Protocol;

/// Config to generate QR code schema.
pub struct Config {
    pub ssid: String,
    pub key: String,
    pub encryption: Protocol,
}

impl Config {
    pub fn new(ssid: String, key: String, encryption: Protocol) -> Self {
        Config {
            ssid,
            key,
            encryption,
        }
    }
}
