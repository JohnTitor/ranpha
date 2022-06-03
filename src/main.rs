use bpaf::*;

mod config;
mod qr;
mod validate;

fn main() {
    let opts = opts();
    crate::qr::generate_qr_code(opts, 256, "./qr.png").unwrap_or_else(|e| {
        eprintln!("failed to generate QR code image: {e}");
        std::process::exit(1);
    });
}

// FIXME: Accept some more flags, e.g.:
// - generated data formats
// - output dir
// - size
#[derive(Clone, Debug)]
pub struct Opts {
    encryption_protocol: String,
    ssid: String,
    key: String,
}

fn opts() -> Opts {
    let encryption_protocol = short('p')
        .long("protocol")
        .help(
            "encryption protocol, WPA2, WPA, or WEP is valid (case-insensitive),\n\
            any other inputs are treated as no key",
        )
        .argument("PROTOCOL")
        .from_str();
    let ssid = short('s')
        .long("ssid")
        .help("SSID of your network")
        .argument("SSID")
        .from_str();
    let key = short('k')
        .long("key")
        .help("key of your network")
        .argument("KEY")
        .from_str();

    let parser = construct!(Opts {
        encryption_protocol,
        ssid,
        key
    });

    Info::default()
        .descr("Generate QR code of your Wi-FI network")
        .for_parser(parser)
        .run()
}
