<div align="center">
<h1> OpenVisionAPI Client </h1>
</div>

## 📦 Setup

### Binary releases

You can download the prebuilt binaries from the [release page](https://github.com/openvisionapi/ova/releases)

### Build from source

To build from the source, you need [Rust](https://www.rust-lang.org/) compiler and
[Cargo package manager](https://doc.rust-lang.org/cargo/).

Once Rust and Cargo are installed, run the following command to build:

```bash
$ cargo build --release
```

This will produce an executable file at `target/release/ova` that you can copy to a directory in your `$PATH`.

## 🚀 Usage

Print the API response in a pretty formatted json :

```bash
$ ova detection -i /path/to/image
```

Draw boxes on the detected objects:

```bash
$ ova detection -v -i /path/to/image
```

## ⚙️ Configuration

The configuration can be set up using the following environment variables:

**OVA_DETECTION_URL** : The URL to the OpenVisionAPI server. The default is `https://api.openvisionapi.com/api/v1/detection`

**OVA_OUTPUT_DIR** : The directory where to store the image. The default is `./output`

## 🤝 Contributing

Your contributions are welcome !

## ⚖️ License

AGPLv3
