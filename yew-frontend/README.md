# Text Cleaner

A fully WASM-based frontend for a simple text manipulation program written in [Rust](https://www.rust-lang.org/)([Yew](https://yew.rs/) + [Tauri](https://docs.rs/tauri/latest/tauri/)). 
It only runs a minimal amount of "glue" js on the browser (at least until we've got this whole [WASM and DOM api interrop thing](https://hacks.mozilla.org/2019/08/webassembly-interface-types/) figured out).

**Note** This project is mostly an excuse for me to look into some Rust-based frontend frameworks.

## Development

### Only the web-based frontend.

The WASM-app [trunk](https://trunkrs.dev/) is one-stop shop for setting up the development environment.
Once installed, we can run `trunk serve` inside `yew-frontend` (i.e. the root of the Yew project) to spin up a live-reload dev server.

### Also the native app.

Install a rust binary called `tauri`, with `cargo install tauri`. 
There may be various C development libraries that it needs to install and then compile the project. It'll show appropriate errors if any lib is missing, in which case, download one for your Linux distribution using `apt`, `dnf`, or `pacman`, etc.

Once it stops complaining, a development server can be spun up by running `cargo tauri dev --verbose` inside the `src-tauri` directory. Same as `trunk`, it will also set up live-reload for you.

## Usage

### Docker

```sh
docker run -d -p 8080:80 --name text-cleaner aalekhpatel07/text-cleaner:1.0.0
```

Then visit `http://localhost:8080` to view the app.

### Deploy

For web-based deployments, it is basically packaged into an nginx container already so feel free to use straight up. Otherwise, Tauri config (the `src-tauri`) directory might need some fiddling to get Tauri to work the correct way and actually build "natively" (kinda like Electron but less junk) for all major platforms.

**Note**: The last time I tried, `cargo tauri build --verbose` fails with a mysterious file permission issue. Once fixed, that should automatically generate all build artifacts  for most major platforms (i.e. `.deb`, `.dmg`, `AppImage`, `.msi`, etc).
