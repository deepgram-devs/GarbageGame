Compiling `garbage-game-lib` for WASM is non-trivial. Instructions for doing this are here:

https://godot-rust.github.io/book/exporting/html5.html

However, I found some tiny differences and specific details which I needed to do to get
the instructions to work. For one, I needed to get this specific release of emscripten
(as recommended by the godot-rust book guide: `Emscripten 3.1.21-git (3ce1f726b449fd1b90803a6150a881fc8dc711da)`):

https://github.com/emscripten-core/emsdk/releases/tag/3.1.21

Also, I needed to run the following _instead_ of `./emsdk install tot`:

```
./emsdk install 3.1.21

```

This may be obvious for someone familiar with emscripten, but it was not for me. Essentially,
even if downloading the 3.1.21 release, if one uses `tot` (tip-of-tree), they end up installing
a later version of emscripten - and that will have compatibility issues.

Next, I was not able to compile the Rust code with the following:

```
cargo +nightly build --target=wasm32-unknown-emscripten --release
```

Indeed, one must specify the specific nightly version recommended by the godot-rust book guide (`rustc 1.65.0-nightly (8c6ce6b91 2022-09-02)`).
However, even doing that did not work for me:

```
cargo +nightly-2022-09-02 build --target=wasm32-unknown-emscripten --release
```

So how did I get it to work? Well, I made `rust-toolchain.toml` with the following contents:

```
[toolchain]
channel = "nightly-2022-09-02"
components = [ "rustfmt", "clippy" ]
```

And then I could compile the WASM version with:

```
cargo build --target=wasm32-unknown-emscripten --release
```

Compiling the non-WASM version was pretty normal (only setup for Linux):

```
cargo build --release
```
