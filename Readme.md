<p align="center"><a href="https://github.com/foundpatterns/torchbear"><img width="100" src="https://i.imgur.com/3GfOkqo.png" alt="torchbear logo"></a><br>This Project is Currently in Stealth Mode.<br>please do not post a news story until v1 is released very shortly.<br>thank you.</p>

<p align="center">
  <a href="#get-started"><img width="70" src="https://image.flaticon.com/icons/svg/174/174836.svg" alt="android logo"></a>
  &nbsp;&nbsp;
  <a href="#get-started"><img width="70" src="https://image.flaticon.com/icons/svg/179/179331.svg" alt="windows logo"></a>
  &nbsp;&nbsp;
  <a href="#get-started"><img width="70" src="https://image.flaticon.com/icons/svg/732/732181.svg" alt="macos logo"></a>
  &nbsp;&nbsp;
  <a href="#get-started"><img width="70" src="https://i.imgur.com/T4Aa1rb.png" alt="linux logo"></a>
</p>

<p align="center">
  user badges
  <br>
  <a href="https://github.com/rust-lang/crates.io/issues/704"><img src="https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg" alt="Actively Maintained"></a>
  <a href="https://opensource.com/life/16/1/8-ways-contribute-open-source-without-writing-code"><img src="https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=" alt="Contributions Welcome"></a>
  <a href="https://akrabat.com/the-beginners-guide-to-contributing-to-a-github-project/#to-sum-up"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome"></a>
  <a href="https://en.wikipedia.org/wiki/List_of_parties_to_international_copyright_agreements"><img src="https://img.shields.io/badge/License-MIT%2FApache2-blue.svg" alt="License: MIT/Apache"></a>
  <a href="https://discord.gg/b6MY7dG"><img src="https://img.shields.io/badge/chat-on%20discord-7289da.svg" alt="Chat"></a>  <br>
  developer badges
  <br>
  <a href="https://www.travis-ci.com/foundpatterns/torchbear"><img src="https://travis-ci.com/foundpatterns/torchbear.svg?branch=master" alt="Travis Build Status"></a>
  <a href="https://ci.appveyor.com/project/mitchtbaum/torchbear"><img src="https://ci.appveyor.com/api/projects/status/mg6e0p7s5v7j61ja?svg=true" alt="Appveyor Build Status"></a>
  <a href="https://deps.rs/crate/torchbear"><img src="https://deps.rs/repo/github/foundpatterns/torchbear/status.svg" alt="Dependencies"></a>
  <a href="https://crates.io/crates/torchbear"><img src="https://img.shields.io/crates/v/torchbear.svg" alt="torchbear Crate"></a>
  <a href="https://github.com/foundpatterns/torchbear/releases"><img src="https://img.shields.io/github/downloads/foundpatterns/torchbear/total.svg" alt="Download Total"></a>
</p>

Torchbear gives you a simple, yet powerful, general-purpose application development environment.  It can help you do web automation, embedded programming, data analysis, numerical computing.. really anything you can imagine.

It's built in [Rust](https://rust-lang.org), but you don't need to learn Rust to use Torchbear, because it aims to give you the simplest possible programming environment with the most powerful addon libraries.  Scratch, BASIC, Lua, Python, Ruby, NodeJS etc developers should feel right at home... and with what one might call "more bells and whistles" - or just `complete`.

Those without a programming background can [learn Lua in 15 minutes](http://tylerneylon.com/a/learn-lua/) to be able to "speak" its programming langauge (its syntax and notation).  Then you can also access Torchbear's functions, which are described below and viewable in the [bindings' documentation](https://foundpatterns.github.io/torchbear-docs/index.html) (generated using our [code map](http://github.com/foundpatterns/lua-module-map) app).

Read on for more about Torchbear's functions.  **Note:** every module is completely built-in, and they're written in pure Rust, or they otherwise wrap a [C](https://en.wikipedia.org/wiki/C_(programming_language)) library, safely (noted with an asterisk*).

## Built-in Modules

#### Environment
* [Actix](https://github.com/actix/actix) actor-model [concurrency](http://berb.github.io/diploma-thesis/original/023_concurrency.html#models) framework (see [RFC 613](https://github.com/rust-lang/rfcs/issues/613) for more info)
* [Actix Lua](https://github.com/poga/actix-lua) safe Lua scripting environment for Actix
* [rlua](https://github.com/kyren/rlua)* Lua 5.3.5 (also with tools for traceback error messages, logs, and performance profiling)
* [Actix Web](https://github.com/actix/actix-web) HTTP/1.x and HTTP/2.0 web servers and clients

#### System
* [Standard filesystem](https://doc.rust-lang.org/std/fs/index.html) operations plus recursive copying
* [Standard directory](https://github.com/soc/dirs-rs) cross-platform config, cache, and data paths
* [Standard process](https://doc.rust-lang.org/std/process/index.html) environment inspection and manipulation
* [Standard environment](https://doc.rust-lang.org/std/process/index.html) terminating to abort and exit the current process
* [Standard memory](https://doc.rust-lang.org/std/io/struct.Cursor.html) querying, aligning, initializing, and manipulating
* [Chrono](https://github.com/chronotope/chrono) time/date generation and verification

#### Crypto
* [Libsodium](https://github.com/sodiumoxide/sodiumoxide)* cryptographic signing and verifying, and encrypting and decrypting
* [Blake2](https://github.com/RustCrypto/hashes) cryptographic hash function
* [Checksumdir](https://github.com/sh-zam/checksumdir) deterministic directory hashing

#### Number
* [libm](https://en.wikipedia.org/wiki/C_mathematical_functions)* mathematical functions, like exponential, power, trigonometric, hyperbolic, floating-point manipulation, classification

#### String
* [UUID-rs](https://github.com/uuid-rs/uuid) UUID generation and verification
* [ulid](https://github.com/dylanhart/ulid-rs) Universally Unique Lexicographically Sortable Identifier
* [Heck](https://github.com/withoutboats/heck) case conversions
* [regex](https://github.com/rust-lang/regex) matching and replacing
* [MIME](https://github.com/abonander/mime_guess) type guessing

#### Structured Text
* [SCL](https://github.com/Keats/scl) safe serializing/deserializing
* [YAML](https://en.wikipedia.org/wiki/YAML#Example) serializing and deserializing using [serde](https://github.com/serde-rs/serde)
* [JSON](https://en.wikipedia.org/wiki/JSON#Example) serializing and deserializing using [serde](https://github.com/serde-rs/serde)
* [Tantivy](https://github.com/tantivy-search/tantivy) searching, indexing, schema building, and document adding, updating, and deleting

#### Unstructured Text
* [Tera](https://github.com/Keats/tera) [template rendering](https://tera.netlify.com/docs/installation/) similar to [Jinja lang](http://jinja.pocoo.org/docs/2.10/)
* [Comrak](https://github.com/kivikakk/comrak) [Markdown](https://en.wikipedia.org/wiki/Markdown) to HTML outputting
* [Tantivy](https://github.com/tantivy-search/tantivy) searching, indexing, schema building, and document adding, updating, and deleting
* [Select-rs](https://github.com/utkarshkukreti/select.rs) HTML scraping

#### Archive
* [Zip](https://github.com/mvdnes/zip-rs) file decompression
* [Tar](https://github.com/alexcrichton/tar-rs) file decompression
* [xz](https://github.com/alexcrichton/xz2-rs) lzma file decompression

#### Diff
* [Diff](https://github.com/foundpatterns/diff-rs) [unidiff](https://en.wikipedia.org/wiki/Diff#Unified_format) generation using strings and text files 
* [Patch](https://github.com/foundpatterns/patch-rs) unidiff processor for text files
* [Split Diff](https://github.com/foundpatterns/splitdiff-rs) to break unidiff files into multiple
* [List Diff](https://github.com/foundpatterns/lsdiff-rs) to list files affected by a unidiff
* [Interdiff](https://stackoverflow.com/questions/30603958/what-does-interdiff-do-that-diff-cannot)** help wanted to compare diff files
* [Git](https://github.com/alexcrichton/git2-rs)* cloning, pulling, repo creation, staging, committing, and log access (builtin, no `git` dependency)

## Additional Lua Libraries and Frameworks

* [Torchbear Libs](https://github.com/foundpatterns/torchbear-libs) · libraries for logging, terminal coloring, event triggering, functional programming, graph data processing, argument parsing (todo), and file patching (todo)
* [Torchbear Libs Dev](https://github.com/foundpatterns/torchbear-libs-dev) · a library for inspecting tables
* [ContentDB](https://github.com/foundpatterns/contentdb) · a document-oriented, file-based database (see also [ContentDB-Lua](https://github.com/foundpatterns/contentdb-lua))
* [Lighttouch](https://github.com/foundpatterns/lighttouch) · a simple, event-driven, rule-based, dynamically-loaded, functional, parameter-populated, configurable, version-controlled application framework

## Get Started

Torchbear comes as a single executable (eg. as a binary `.exe` file) which makes it easy to install and easy to run apps.  It also comes with a package manager, called [Machu Picchu](https://github.com/foundpatterns/machu-picchu), which helps you to download more apps; it also works as a dependency manager, making deploying and developing component-oriented software easy.

### Install

To install Torchbear, run this command using your terminal ([what's a terminal?](#what-is-a-terminal)):

```sh
 curl https://git.io/fpcV6 -sSfL | bash
```

[The installer](https://github.com/foundpatterns/torchbear/blob/master/install.sh) automatically gets the latest version (which is also available on [Torchbear's GitHub releases page](https://github.com/foundpatterns/torchbear/releases)) and puts it in a convenient file location for you.  To do this, it downloads the latest zip file for your operating system and hardware architecture, then it unzips the executable to a place where it will run as a command.  You can do that manually or differently, as you like.

#### What is a terminal?

If you haven't heard of a terminal before, here's a [1 min intro to what is a terminal window](https://www.youtube.com/watch?v=zw7Nd67_aFw).  This works on Android, Windows, MacOS, and Linux devices very similarly, but you might need one other tool first:

* Android: install [Termux](https://termux.com/).

* Windows: install [Cmder](http://cmder.net/) Full (within your user directory, eg not in Program Files).

* MacOS:  mostly ready out-of-the-box.

* Linux: mostly ready out-of-the box.

* iOS: work in progress, see [Torchbear iOS build (iPhone and iPad)](https://github.com/foundpatterns/torchbear/issues/120)

* [Redox](https://redox-os.org/): work in progress, see [Torchbear Redox release](https://github.com/foundpatterns/torchbear/issues/18)

### Uninstall
```sh
curl https://git.io/fpcV6 -sSfL | bash -s -- --uninstall
```
or, if you have `install.sh` already downloaded:
```sh
bash install.sh --uninstall
```

## Development

#### Rust Development

You can compile from source by [installing Cargo](https://www.rust-lang.org/tools/install) (Rust's package manager) and installing `torchbear` using Cargo:

`cargo install torchbear`

Compiling from this repository also works similarly:

```
git clone https://github.com/foundpatterns/torchbear
cd torchbear
cargo build --release
```

Compilation will probably take several minutes depending on your machine and your network speed. The binary will end up in `./target/release/torchbear`.

#### Hello World App

- put in `init.lua`:

```lua
#!/usr/bin/env torchbear

print("hello from Torchbear")
```

- make it executable with `chmod +x init.lua`

- then run it with `./init.lua`

## App Stores

* [Found Patterns Studio App Store](https://github.com/foundpatterns/app-store) (installed by default)
* yours?

## Contributors wanted

Torchbear extends Rust's growing library ecosystem. Developers and users alike are welcomed to participate in [Torchbear's issue queue](https://github.com/foundpatterns/torchbear/issues) for small changes and high impact contributions.  Everyone is invited.

Even moderately experienced Rust developers can work on [adding bindings](https://github.com/foundpatterns/torchbear/labels/feature%2Fbindings) or adding other functions.  There are many examples to learn from in the bindings directory.  Through this process, you'll learn a Rust library's API inside and out, and you'll put another tool into the hands of a thriving userbase.

Experienced Rust developers' reviews would be greatly appreciated, eg those familiar with low-level library idioms and especially those well-versed in [Actix](https://github.com/actix/actix).  Much of the power functionality built-in to Torchbear comes from libraries like Actix, Actix-Web, [Actix-Lua](https://github.com/poga/actix-lua), [rlua](https://github.com/kyren/rlua), and many more well-picked ones which need thorough review and careful analysis to make a good programming environment.

Users who who want to add a 'review' or 'story' about your use cases, simply add this "issue label" (type/review) or (type/story).  Everyone is welcomed to do so, and this will help users and developers understand Torchbear from eachother's points of view.  Developers who want to post other feedback and analysis will receive a hearty thank you.
