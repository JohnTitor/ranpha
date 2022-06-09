use bpaf::*;
use validate::{ImageFormat, Protocol};

mod config;
mod qr;
mod validate;

fn main() {
    let opts = opts();
    let outdir = opts.output.clone();
    let format = opts.image_format;
    crate::qr::generate_qr_code(&opts, opts.size, &format!("{outdir}qr.{format}")).unwrap_or_else(
        |e| {
            eprintln!("failed to generate QR code image: {e}");
            std::process::exit(1);
        },
    );
}

#[derive(Clone, Debug)]
pub struct Opts {
    encryption_protocol: Protocol,
    ssid: String,
    key: Option<String>,
    image_format: ImageFormat,
    output: String,
    size: usize,
}

fn opts() -> Opts {
    let encryption_protocol = short('p')
        .long("protocol")
        .help("encryption protocol, only WPA2, WPA, or WEP is valid (case-insensitive).")
        .argument("PROTOCOL")
        .from_str::<Protocol>();
    let ssid = short('s')
        .long("ssid")
        .help("SSID of your network.")
        .argument("SSID");
    let key = short('k')
        .long("key")
        .help("key of your network. \"nopass\" will be used if not specified.")
        .argument("KEY")
        .optional();
    let image_format = short('f')
        .long("format")
        .help(
            "image format of generated image. Only PNG or SVG is valid (case-insensitive). \
            PNG will be used if not specified.",
        )
        .argument("IMAGE_FORMAT")
        .from_str::<ImageFormat>()
        .fallback(ImageFormat::Png);
    let output = short('o')
        .long("output")
        .help("output path of generated image. The current dir will be used if not specified.")
        .argument("OUT_DIR")
        .fallback("./".to_string());
    let size = long("size")
        .help("size of generated image (px). 128 will be used if not specified.")
        .argument("SIZE")
        .from_str::<usize>()
        .fallback(128);

    let parser = construct!(Opts {
        encryption_protocol,
        ssid,
        key,
        image_format,
        output,
        size,
    });

    Info::default()
        .descr("Generate QR code of your Wi-FI network")
        .for_parser(parser)
        .run()
}
