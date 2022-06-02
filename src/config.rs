enum EncryptionProtocol {
    Wpa2,
    Wpa,
    Wep,
    NoPass,
}

impl std::fmt::Display for EncryptionProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Wpa2 => "WPA2",
            Wpa => "WPA",
            Wep => "WEP",
            NoPass => "nopass",
        };
        write!(f, "{}", str)
    }
}

pub struct Config {
    pub ssid: String,
    pub key: String,
    pub encryption: String,
}

impl Config {
    pub fn new(ssid: String, key: String, encryption: String) -> Self {
        Config { ssid, key, encryption }
    }
}
