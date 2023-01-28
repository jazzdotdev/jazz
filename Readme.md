Jazz is a language that makes programming embedded, web, apps etc easier using a simple scripting language with the tools you need built-in by the Rust and Lua communities.  (It's currently being renamed from Torchbear.)

## Blessed Rust Modules

As you wish to solve more use cases with your programs, you can access [Torchbear's functions](#built-in-modules), which are described below and viewable in the [bindings' documentation](https://foundpatterns.github.io/torchbear-docs/index.html) (generated using our [code map](http://github.com/foundpatterns/lua-module-map) app).

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
* [BLAKE2](https://github.com/RustCrypto/hashes) cryptographic hash function
* [BLAKE3](https://github.com/BLAKE3-team/BLAKE3) cryptographic hash function
* [Checksumdir](https://github.com/sh-zam/checksumdir) deterministic directory hashing

#### Number
* [libm](https://en.wikipedia.org/wiki/C_mathematical_functions)* mathematical functions, like exponential, power, trigonometric, hyperbolic, floating-point manipulation, classification

#### String
* [NanoID](https://github.com/nikolay-govorov/nanoid) secure, URL-friendly, unique string ID generator
* [UUID-rs](https://github.com/uuid-rs/uuid) UUID generation and verification
* [ulid](https://github.com/dylanhart/ulid-rs) Universally Unique Lexicographically Sortable Identifier
* [Case conversions](https://github.com/withoutboats/heck) to CamelCase, snake_case, kebab-case, SHOUTY_SNAKE_CASE, mixedCase, and Title Case
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
* [xz](https://github.com/alexcrichton/xz2-rs) lzma file compression and decompression

#### Diff
* [Diff](https://github.com/foundpatterns/diff-rs) generating [diffs](https://en.wikipedia.org/wiki/Diff#Unified_format) using strings and text files 
* [Patch](https://github.com/foundpatterns/patch-rs) applies diffs to strings and text files
* [Split Diff](https://github.com/foundpatterns/splitdiff-rs) breaks individual diffs into multiple diffs, per file
* [List Diff](https://github.com/foundpatterns/lsdiff-rs) lists files affected by a diff
* [Interdiff](https://github.com/changeutils/interdiff-rs)**
* [Git](https://github.com/alexcrichton/git2-rs)* cloning, pulling, repo creation, staging, committing, and log access (builtin, no `git` dependency)

Note* (asterisk) means safely wraps a [C library](https://en.wikipedia.org/wiki/C_(programming_language))

Note** (two asterisks) means help wanted

## Blessed Libraries and Frameworks

* [Torchbear Libs](https://github.com/foundpatterns/torchbear-libs) · libraries for logging, terminal coloring, event triggering, functional programming, graph data processing, argument parsing (todo), and file patching (todo)
* [Torchbear Libs Dev](https://github.com/foundpatterns/torchbear-libs-dev) · a library for inspecting tables
* [ContentDB](https://github.com/foundpatterns/contentdb) · a document-oriented, file-based database
* [Lighttouch](https://github.com/foundpatterns/lighttouch) · a simple, event-driven, rule-based, dynamically-loaded, functional, parameter-populated, configurable, version-controlled application framework

## Get Started

Jazz comes as a single executable (eg. as a binary `.exe` file) which makes it easy to install and easy to run apps.  It also comes with a package manager, called [Machu Picchu](https://github.com/foundpatterns/machu-picchu), which helps you to download more apps.

### Install

To install Jazz on Android or Windows, run this command using your terminal ([what's a terminal?](#what-is-a-terminal)):

```sh
 curl https://git.io/fpcV6 -sSfL | bash
```

MacOS and Linux users will need admin permissions to run:

```sh
 curl https://git.io/fpcV6 -sSfL | sudo bash
```

[The installer](https://github.com/foundpatterns/torchbear/blob/master/install.sh) automatically gets the latest version (which is also available on [Torchbear's GitHub releases page](https://github.com/foundpatterns/torchbear/releases)) and puts it in a convenient file location for you.  To do this, it downloads the latest zip file for your operating system and hardware architecture, then it unzips the executable to a place where it will run as a command.  You can do that manually or differently, as you like.

#### What is a terminal? (command prompt)

If you haven't heard of a terminal before, here's a [1 min intro to what is a terminal window](https://www.youtube.com/watch?v=zw7Nd67_aFw).  This works on Android, Windows, MacOS, and Linux devices very similarly, but you might need one other tool first:

* Android: install [Termux](https://termux.com/).

* Windows: install [Cmder](http://cmder.net/) Full (within your user directory, eg not in Program Files).

* MacOS:  mostly ready out-of-the-box.

* Linux: mostly ready out-of-the box.

* iOS: work in progress, see [Torchbear iOS build (iPhone and iPad)](https://github.com/foundpatterns/torchbear/issues/120)

* [Redox](https://redox-os.org/): work in progress, see [Torchbear Redox release](https://github.com/foundpatterns/torchbear/issues/18)

### Update

To update, run `torchup`.

### Uninstall

To uninstall, run `torchup --uninstall`.

## Development

#### Hello World App

1. put this in `init.lua`:

```lua
#!/usr/bin/env speakeasy

print("hello from Torchbear")
```

2. make it executable with `chmod +x init.lua`

3. then run it with `./init.lua`

**Note:** [Machu Picchu](https://github.com/foundpatterns/machu-picchu) also works as a dependency manager, making deploying and developing component-oriented software easy.  Check how other projects use it until more documentation is available.  Also can start finding some useful library scripts for your programs in our [Lunar Transit](https://github.com/lunar-transit) account.

#### Rust Development

You can compile from source by [installing Cargo](https://www.rust-lang.org/tools/install) (Rust's package manager) and installing `torchbear` using Cargo:

`cargo install torchbear`

Compiling from this repository also works similarly:

```
git clone https://github.com/foundpatterns/torchbear
cd torchbear
cargo build --release
```

Note: You will also need a C compiler, eg on Debian-based Linux distros `sudo apt install build-essential`.  Compilation will probably take several minutes depending on your machine and your network speed. The binary will end up in `./target/release/torchbear`.

## App Stores

* [Found Patterns Studio App Store](https://github.com/foundpatterns/app-store) (installed by default)
* yours?

## Contributors wanted

Torchbear extends Rust's growing library ecosystem. Developers and users alike are welcomed to participate in [Torchbear's issue queue](https://github.com/foundpatterns/torchbear/issues) for small changes and high impact contributions.  Everyone is invited.

Even moderately experienced Rust developers can work on [adding bindings](https://github.com/foundpatterns/torchbear/labels/feature%2Fbindings) or adding other functions.  There are many examples to learn from in the bindings directory.  Through this process, you'll learn a Rust library's API inside and out, and you'll put another tool into the hands of a thriving userbase.

Experienced Rust developers' reviews would be greatly appreciated, eg those familiar with low-level library idioms and especially those well-versed in [Actix](https://github.com/actix/actix).  Much of the power functionality built-in to Torchbear comes from libraries like Actix, Actix-Web, [Actix-Lua](https://github.com/poga/actix-lua), [rlua](https://github.com/kyren/rlua), and many more well-picked ones which need thorough review and careful analysis to make a good programming environment.

Users who who want to add a 'review' or 'story' about your use cases, simply add this "issue label" (type/review) or (type/story).  Everyone is welcomed to do so, and this will help users and developers understand Torchbear from eachother's points of view.  Developers who want to post other feedback and analysis will receive a hearty thank you.

## Community

Eveyone interested in learning and solving problems with programming and open-source tools in general is welcomed to come to [Found Patterns Studio's Discord Server](https://discord.gg/f6XSuWs) to meet fellow engineers and to work together on team-driven projects, like Torchbear.  If you haven't heard of [Discord](https://discordapp.com/) before, you could start with these videos, [Discord For Dummies: Basic Use and Set up Instructions for Discord App](https://www.youtube.com/watch?v=7BFytSpuAWs) and [How To Setup And Use Discord - Basic Overview Of Features and Tools](https://www.youtube.com/watch?v=E7xznRGg9WM).  We use it to help eachother use and build Torchbear and many other projects.  It's a safe and friendly place to get things done.

## Supporters

As an open source project, continued development depends on support from people like you.  To see current and past supporters, visit our [supporters.md](https://github.com/foundpatterns/torchbear/blob/master/supporters.md) file.

To add your support to this project, as it is currently lead, and to join that list, visit [Mitchell's contact and support channel](https://github.com/naturallymitchell/call-me-maybe) for more info.

## Sponsors

To sponsor this project and put your logo here, visit [Mitchell's GitHub sponsors page](https://github.com/sponsors/naturallymitchell).

## Found Patterns Museum & Exhibit Sponsors

This project is currently in early development for a future exhibit in [Found Patterns Museum](https://github.com/lighttouch-apps/found-patterns-museum), called [User Handed Programming](https://github.com/foundpatterns/studio-care/issues?q=label%3Aexhibit%2Fuser-handed-programming).  If you would like to help us realize it, [make a donation to Found Patterns Museum](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=3MDKYD7P9AKE2&source=url).

### Thank you · Namaste · Aloha 👋
