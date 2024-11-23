# gnucash-toolbox

A toolbox to work with GnuCash databases.

*This is a work in progress, please look at the `develop` branch for ongoing
development.*

## Setup

### Installation with Nix

If you are a **Nix** user on **Linux** or **macOS**, you can add
`gnucash-toolbox` to your user profile by running:

    nix profile install github:ejpcmac/gnucash-toolbox

### Installation from the Debian package

If you are a **Debian** user—or of derivatives like **Ubuntu**—, you can install
`gnucash-toolbox` by running:

    curl -OL https://github.com/ejpcmac/gnucash-toolbox/releases/download/v0.1.0/gnucash-toolbox_0.1.0-1_amd64.deb
    sudo apt update
    sudo apt install ./gnucash-toolbox_0.1.0-1_amd64.deb

### Installation from the MSI package

If you are a **Windows** user, you can download an MSI package on the [the
release
page](https://github.com/ejpcmac/gnucash-toolbox/releases/latest)
and install it. You may need to allow its execution by doing *Right Click >
Properties*, then checking the *Unblock* box in the security section at the
bottom of the page.

### Installation from a pre-built binary

If you are a user of any other **Linux** distribution, **macOS** or **Windows**,
you can download a statically-linked executable on [the release
page](https://github.com/ejpcmac/gnucash-toolbox/releases/latest). Just rename
it to `gnc`—or `gnc.exe` on Windows—and put it somewhere in your `PATH`.

### Installation with Cargo

If you are a **Rust programmer**, you can install `gnc` by running:

    cargo install gnc-cli

## Building an installer

### Linux (Debian)

From inside a Nix devshell, you can run:

    $ build-deb

You should then find a Debian package in
`target/x86_64-unknown-linux-musl/debian/`.

### Windows

With a Rust toolchain installed on your machine, you can:

1. Install [WiX v3](https://wixtoolset.org/docs/wix3/).

2. Run:

        > cargo install cargo-wix
        > cargo wix --package gnc-cli --nocapture

You should find an installer in `target/wix/`.

## [Contributing](CONTRIBUTING.md)

Before contributing to this project, please read the
[CONTRIBUTING.md](CONTRIBUTING.md).

## License

Copyright © 2024 Jean-Philippe Cugnet

This project is licensed under the [GNU General Public License 3.0](LICENSE).
