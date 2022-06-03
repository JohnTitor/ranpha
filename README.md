# ranpha

Generate QR code of your Wi-Fi network.

```
Usage: -p PROTOCOL -s SSID [-k KEY] [-f IMAGE_FORMAT] [-o OUT_DIR] [--size SIZE]

Available options:
    -p, --protocol <PROTOCOL>    encryption protocol, only WPA2, WPA, or WEP is valid (case-insensitive).
    -s, --ssid <SSID>            SSID of your network.
    -k, --key <KEY>              key of your network. "nopass" will be used if not specified.
    -f, --format <IMAGE_FORMAT>  image format of generated image. Only PNG or SVG is valid (case-insensitive). PNG will be used if not specified.
    -o, --output <OUT_DIR>       output path of generated image. The current dir will be used if not specified.
        --size <SIZE>            size of generated image (px). 128 will be used if not specified.
    -h, --help                   Prints help information
```
