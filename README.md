# hdump

Dumps a binary/text file as hex strings.

## Installation and Usage

### Installation

The [x64 Windows executable](https://github.com/abhinavgunwant/hdump/releases/download/v0.1.0/hdump_win_x64.zip) can be downloaded in the [Releases](https://github.com/abhinavgunwant/hdump/releases) page. Just check the zip file in the "Assets" section.

Executable for other Operating Systems will be available later on but for now you can refer to the "Building" section below.

### Usage

```bash
hdump file-name.ext
```

## Building

To build, you need to install rust toolchain from https://rustup.rs/.

Clone this repository and cd into the cloned directory and then execute:

```bash
cargo build --release
```

