<p align="center"><img width="100" src="https://i.imgur.com/3GfOkqo.png" alt="torchbear logo"></p>

<p align="center">
  <a href="https://www.travis-ci.com/foundpatterns/torchbear"><img src="https://travis-ci.com/foundpatterns/torchbear.svg?branch=master" alt="Travis Build Status"></a>
  <a href="https://ci.appveyor.com/project/mitchtbaum/torchbear"><img src="https://ci.appveyor.com/api/projects/status/mg6e0p7s5v7j61ja?svg=true" alt="Appveyor Build Status"></a>
  <a href="https://crates.io/crates/torchbear"><img src="https://img.shields.io/crates/v/torchbear.svg" alt="torchbear Crate"></a>
  <a href="https://discord.gg/sWCQxT"><img src="https://img.shields.io/badge/chat-on%20discord-7289da.svg" alt="Chat"></a>
  <a href="https://github.com/foundpatterns/torchbear/issues"><img src="https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=" alt="Contributions Welcome"></a>
</p>

torchbear 🐻 is an extremely fast and featureful Lua application framework.  It gives you power of Rust with the simplicity of Lua.

* *HTTP/1.x* and *HTTP/2.0* web servers and clients on [Actix Web](https://github.com/actix/actix-web)
* Jinja template rendering with [Tera](https://github.com/Keats/tera)
* Markdown output using [Comrak](https://github.com/kivikakk/comrak)
* Signatures and Encryption from [Libsodium](https://github.com/maidsafe/rust_sodium)
* Filesystem operations from [`std::fs::*`](https://doc.rust-lang.org/std/fs/index.html)
* Set-theoretic operations using [`std::collections::HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
* YAML and JSON serializing/deserializing with [Serde](https://github.com/serde-rs/serde)
* UUID generation and verification with [UUID-rs](https://github.com/uuid-rs/uuid)
* HTML scraping with [Select-rs](https://github.com/utkarshkukreti/select.rs)
* Time/Date generation and verification using [Chrono](https://github.com/chronotope/chrono)
* Git repo creation, commit staging, and log access from [libgit2](https://github.com/alexcrichton/git2-rs)

# Example
Download [torchbear static webserver](https://github.com/foundpatterns/torchbear-static-webserver)

Run `torchbear` inside
