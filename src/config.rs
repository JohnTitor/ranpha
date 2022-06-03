/// Config to generate QR code schema.
pub struct Config {
    pub ssid: String,
    pub key: String,
    pub encryption: String,
}

impl Config {
    pub fn new(ssid: String, key: String, encryption: String) -> Self {
        Config {
            ssid,
            key,
            encryption,
        }
    }
}
