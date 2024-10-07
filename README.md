<h1 align="center">
    will
</h1>

[<img alt="github" src="https://img.shields.io/badge/github-uraneko.will-A5915F?style=for-the-badge&logo=github&labelColor=3a3a3a" height="25">](https://github.com/uraneko/will) 
[<img alt="crates.io" src="https://img.shields.io/crates/v/will.svg?style=for-the-badge&color=E40046&logo=rust&labelColor=3a3a3a" height="25">](https://crates.io/crates/will) 
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-will-495c9f?style=for-the-badge&logo=docsdotrs&labelColor=3a3a3a" height="25">](https://docs.rs/will) 
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/uraneko/will/rust-ci.yml?branch=main&style=for-the-badge&labelColor=3a3a3a" height="25">](https://github.com/uraneko/will/actions?query=branch%3Amain)
[<img alt="license" src="https://img.shields.io/github/license/uraneko/will?style=for-the-badge&labelColor=3a3a3a&color=ECD53F" height="25">](https://github.com/uraneko/will/blob/main/LICENSE)

will is a homeserver made for my personal use, I will be adding whatever features I need in my homeserver to it. I intend to add some form of plugin system down the line tho, so anyone can add their desired features. 

## State
This project is still in development and doesn't have a working 0.1.0 version yet.

## Features 
- basic http server 

- builtin file explorer with a shared directory that all server users on the LAN have access to.

✗ builtin calendar with events features 

✗ plugins api

<br>
✗ not yet implemented 

~ not yet implemented, low priority.

- work in progress

✓ implemented 

! implemented but buggy

## Installation
## cargo

> [!IMPORTANT] 
> This is not yet implemented.

```bash 
cargo install will --locked
```

## From Source
```bash 
git clone https://github.com/uraneko/will
cd will
cargo build -r --locked
# binary should be found under ./target/release/will
```

### Examples 
```bash
# this starts the home server on port 4567, accessible to all devices on the LAN from the ip address of this device e.g., 192.168.1.105:4567 and on localhost:4567 for the device  the server is on
will --port 4567 --host bidi
```

> [!IMPORTANT] 
> Follows the [SemVer Spec](https://semver.org/) versioning scheme.
> Until the crate hits version 1.0.0, there are no rules, nonetheless, I'll try to make sense.

