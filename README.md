# Tauri Dominator (Project Template)
A [Rust-Dominator](https://github.com/Pauan/rust-dominator) based setup for Tauri, creates a minimal Tauri app geared towards high performance using WASM.

## Development Setup
We depend on Rust [Trunk](https://trunkrs.dev/) to build the application and serve a dev version with hot-reload.
Install Tunk
```bash
cargo install --locked trunk
```

(Tauri is assumed to already be setup, [See here to install Tauri](https://tauri.studio/en/docs/getting-started/intro))
## Run Development
```bash
cargo tauri dev
```

## Build
Remember to set Trunk to build in Release mode from Trunk.toml
> release = false

```bash
$ cargo tauri build
```