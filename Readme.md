<p align="center"><img width="100" src="https://i.imgur.com/3GfOkqo.png" alt="torchbear logo"><br>This Project is Currently in Stealth Mode.<br>please do not post a news story until v1 is released very shortly.<br>thank you.</p>

<p align="center">
  <a href="https://www.travis-ci.com/foundpatterns/torchbear"><img src="https://travis-ci.com/foundpatterns/torchbear.svg?branch=master" alt="Travis Build Status"></a>
  <a href="https://ci.appveyor.com/project/mitchtbaum/torchbear"><img src="https://ci.appveyor.com/api/projects/status/mg6e0p7s5v7j61ja?svg=true" alt="Appveyor Build Status"></a>
  <a href="https://deps.rs/crate/torchbear/0.5.0"><img src="https://deps.rs/crate/torchbear/0.5.0/status.svg" alt="Dependencies"></a>
  <a href="https://crates.io/crates/torchbear"><img src="https://img.shields.io/crates/v/torchbear.svg" alt="torchbear Crate"></a>
  <a href="https://github.com/foundpatterns/torchbear/releases"><img src="https://img.shields.io/github/downloads/foundpatterns/torchbear/total.svg" alt="Download Total"></a>
  <br>
  <a href="https://github.com/rust-lang/crates.io/issues/704"><img src="https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg" alt="Actively Maintained"></a>
  <a href="https://opensource.com/life/16/1/8-ways-contribute-open-source-without-writing-code"><img src="https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=" alt="Contributions Welcome"></a>
  <a href="https://akrabat.com/the-beginners-guide-to-contributing-to-a-github-project/#to-sum-up"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome"></a>
  <a href="https://en.wikipedia.org/wiki/List_of_parties_to_international_copyright_agreements"><img src="https://img.shields.io/badge/License-MIT%2FApache2-blue.svg" alt="License: MIT/Apache"></a>
  <a href="https://discord.gg/b6MY7dG"><img src="https://img.shields.io/badge/chat-on%20discord-7289da.svg" alt="Chat"></a>
</p>

Torchbear gives you power of Rust with the simplicity of Lua.  You can use it for web automation, embedded programming, data anlysis, and anything else you can imagine.

Lua is a very simple language, [that you can learn in 15 minutes](http://tylerneylon.com/a/learn-lua/).  You don't need to learn Rust to use Torchbear.

## Built-in Tools

* [rlua](https://github.com/kyren/rlua) *completely safe* Lua 5.3.5 with traceback error messages
* [Actix Web](https://github.com/actix/actix-web) HTTP/1.x and HTTP/2.0 web servers and clients
* [Tera](https://github.com/Keats/tera) Jinja template rendering
* [Comrak](https://github.com/kivikakk/comrak) Markdown to HTML outputting
* [Libsodium](https://github.com/maidsafe/rust_sodium) cryptographic signing and verifying, and encrypting and decrypting
* [`std::fs::*`](https://doc.rust-lang.org/std/fs/index.html) filesystem operations
* [`std::collections::HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html) set-theoretic operations
* [Serde](https://github.com/serde-rs/serde) YAML and JSON serializing/deserializing
* [SCL](https://github.com/Keats/scl) simple, safe configuration language serializing/deserializing
* [UUID-rs](https://github.com/uuid-rs/uuid) UUID generation and verification
* [Chrono](https://github.com/chronotope/chrono) time/date generation and verification
* [Select-rs](https://github.com/utkarshkukreti/select.rs) HTML scraping
* [Git](https://github.com/alexcrichton/git2-rs) repo creation, staging, committing, and log access
* [Tantivy](https://github.com/tantivy-search/tantivy) schema building, document adding/updating/deleting, and searching
* [regex](https://github.com/rust-lang/regex) matching and replacing
* [MIME](https://github.com/abonander/mime_guess) type guessing
* [Heck](https://github.com/withoutboats/heck) case conversions
* [Zip](https://github.com/mvdnes/zip-rs) file decompression

## Installation

Torchbear comes as a single executable, making it very easy to install.  Here's a simple command to install it:

```sh
 curl https://git.io/fpcV6 -sSfL | bash
```

Our installer gives you the latest version, which is also available on [Torchbear's GitHub releases page](https://github.com/foundpatterns/torchbear/releases), so you can download the zip file for your operating system and hardware architecture and unzip the executable wherever is most convenient for you.  Our install script is much easier though; you can use it by copying and pasting that line into your terminal, then you'll be able to run `torchbear` in any of your apps.

#### What is a terminal?

If you haven't heard of a terminal before, here's a [1 min intro to what is a terminal window](https://www.youtube.com/watch?v=zw7Nd67_aFw).  On Windows, Android, and MacOS, we've tested with these tools that make a very nice user experience:

Windows: install [Cmder](http://cmder.net/) Full.

Android: install [Termux](https://termux.com/).

MacOS: comes mostly ready, but [Homebrew](https://brew.sh/) has additional tools.

## Examples

#### Hello World App

- in `init.lua`

`print("hello from Torchbear")`

- run `torchbear`

#### [Torchbear Simple Webserver](https://github.com/foundpatterns/torchbear-simple-webserver) · also supports TLS

#### [Lighttouch Application Framework](https://github.com/foundpatterns/lighttouch) · 👍 web development 👍

#### [File Witness](https://github.com/foundpatterns/file-witness) · code signing app

#### [Lua Module Map](https://github.com/foundpatterns/lua-module-map) · Lua diagram code visualization

## Contributions wanted

Torchbear extends Rust's growing ecosystem of libraries. Developers are welcomed to [make small changes](https://github.com/foundpatterns/torchbear/issues?q=is%3Aopen+is%3Aissue+label%3Asize%2F0.25) as well as high impact contributions, like [adding bindings](https://github.com/foundpatterns/torchbear/labels/feature%2Fbindings).  There are many examples to learn from in the bindings directory, each with an interesting history.  You'll learn a Rust library's API inside and out, and you'll put another tool into the hands of a growing userbase.

<p align="center">🛠 on 🌎 with ❤️ and 💰</p>
