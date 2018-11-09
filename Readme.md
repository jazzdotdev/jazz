<p align="center"><img width="100" src="https://i.imgur.com/3GfOkqo.png" alt="torchbear logo"></p>

<p align="center">
  <a href="https://www.travis-ci.com/foundpatterns/torchbear"><img src="https://travis-ci.com/foundpatterns/torchbear.svg?branch=master" alt="Travis Build Status"></a>
  <a href="https://ci.appveyor.com/project/mitchtbaum/torchbear"><img src="https://ci.appveyor.com/api/projects/status/mg6e0p7s5v7j61ja?svg=true" alt="Appveyor Build Status"></a>
  <a href="https://crates.io/crates/torchbear"><img src="https://img.shields.io/crates/v/torchbear.svg" alt="torchbear Crate"></a>
  <a href="https://github.com/foundpatterns/torchbear/issues"><img src="https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=" alt="Contributions Welcome"></a>
  <a href="https://discord.gg/sWCQxT"><img src="https://img.shields.io/badge/chat-on%20discord-7289da.svg" alt="Chat"></a>
</p>

torchbear 🔥🐻 is an extremely fast and featureful Lua application framework.  It gives you power of Rust with the simplicity of Lua.

## Built-in Tools

* [Actix Web](https://github.com/actix/actix-web) *HTTP/1.x* and *HTTP/2.0* web servers and clients
* [Tera](https://github.com/Keats/tera) Jinja template rendering
* [Comrak](https://github.com/kivikakk/comrak) Markdown output
* [Libsodium](https://github.com/maidsafe/rust_sodium) Signatures and Encryption
* [`std::fs::*`](https://doc.rust-lang.org/std/fs/index.html) Filesystem operations
* [`std::collections::HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html) Set-theoretic operations
* [Serde](https://github.com/serde-rs/serde) YAML and JSON serializing/deserializing
* [UUID-rs](https://github.com/uuid-rs/uuid) UUID generation and verification
* [Chrono](https://github.com/chronotope/chrono) Time/Date generation and verification
* [Select-rs](https://github.com/utkarshkukreti/select.rs) HTML scraping
* [libgit2](https://github.com/alexcrichton/git2-rs) Git repo creation, commit staging, and log access
* [The Lua Debug Library](https://www.lua.org/pil/23.html) traceback error messages

## Installation

- Download the [latest torchbear release](https://github.com/foundpatterns/torchbear/releases).
- Unzip it in your application.
- You're good to go.

Windows, Android, and Linux builds available for primary architectures.  MacOS builds still to come.  OS Pacakge Managers support coming soon - contributions welcomed.

Android users, install [Termux](https://termux.com/) for a full Linux envrionment.

## Example

Download [torchbear static webserver](https://github.com/foundpatterns/torchbear-static-webserver), run `torchbear` inside.

in `init.lua`
`print("hello from torchbear")`

in `Settings.toml`
`init = "init.lua"`

run `torchbear`
