# {{project-name}}

## Quick Start

```sh
~$ cd {{project-name}}
~/{{project-name}}$ cargo run -- double 50
```

## Project Structure

This repository is setup as a [cargo workspace][] containing several rust projects:
  * A macro library `<project-name>-codegen`
  * A library `<project-name>` (uses the codegen project)
  * An executable `<project-name>-cli` (uses the library and codegen project)
  * A web-assembly project `<project-name>-wasm` (uses the library and codegen project)

```
├── Cargo.lock
├── Cargo.toml
├── README.md
├── {{project-name}}-codegen
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── {{project-name}}
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── {{project-name}}-cli
│   ├── Cargo.toml
│   └── src
│        └── main.rs
└── {{project-name}}-wasm
    ├── Cargo.toml
    └── src
        └── lib.rs
```
## About

This repository was generated using [cargo-generate][], with [jcpst/rust-utility-template][] as the template.

```sh
cargo install cargo-generate
cargo generate --git nanobot248/rust-universal-template --name {{project-name}}
```

<!-- links -->
[cargo workspace]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[nanobot24/rust-universal-template]: https://github.com/nanobot248/rust-universal-template
