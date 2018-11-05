# torchbear [![](https://img.shields.io/crates/v/torchbear.svg)](https://crates.io/crates/torchbear) [![](https://docs.rs/torchbear/badge.svg)](https://docs.rs/torchbear/) [![](https://travis-ci.com/foundpatterns/torchbear.svg?branch=master)](https://www.travis-ci.com/foundpatterns/torchbear) [![](https://ci.appveyor.com/api/projects/status/mg6e0p7s5v7j61ja?svg=true)](https://ci.appveyor.com/project/mitchtbaum/torchbear) [![](https://img.shields.io/discord/497593709219676176.svg?logo=discord)](https://discord.gg/sWCQxT) [![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/foundpatterns/torchbear/issues)

TorchBear is an extremely fast and featureful Lua application framework.  It gives you power of Rust with the simplicity of Lua.

* *HTTP/1.x* and *HTTP/2.0* web servers and clients using Actix Web
* Markdown output using Comrak
* Jinja template rendering using Tera
* Signatures and Encryption using Libsodium
* Filesystem operations using `std::fs::*`
* Set theoretic operations using `std::collections::HashSet` Stringset
* YAML and JSON serializing/deserializing using Serde
* UUID generation and verification using UUID-rs
* HTML scraping using Select-rs
* Time/Date generation and verification using Chrono

# Example
in `launcher.lua`
```lua
print("Hello from TorchBear")
```
in `settings.toml`
```toml
lua_prelude = "/"
```
Run
`torchbear`
