# Word Clock

This repository consists of

- A `wordclock` business logic crate
- Crates for displaying on various media
  - `yew-wordclock` crate (path `./yew`) for display as webassembly based web application
  - `iced-wordclock`crate (path `./iced`) for display rendered on GPU in a graphic window on Linux, Windows and Mac
  - `pancurses-wordclock` crate (`./pancurses`)  for display in a terminal

## Status

The library crate `wordclock` is covered by CI.

![CI Status](https://github.com/almedso/wordclock-in-rust/actions/workflows/ci.yml/badge.svg)

##  How to run

Make sure your have *cargo, rustc* installed.
For the web assembly you also need to install *trunk*  via `cargo install trunk` and
the *wasm-unknown-unknown* target via `rustup add target wasm-unknown-unknown`

Clone this repository and run the example
```bash
git clone https://github.com/almedso/wordclock.git
cd wordclock
cargo run --example terminal-on
```

Run the iced GPU accelerated variant
```bash
cd ./iced
cargo run
```

Run the terminal variant variant
```bash
cd ./pancurses
cargo run
```


Run the web assembly variant
```bash
cd ./yew
trunk serve --open
```


## TODO

- Add Spanish words?
- Add an example on embedded. (e.g. have a look at:  https://www.instructables.com/THE-WORD-CLOCK/)

****

## Notes:

- LED-Strip variant needs 5V and 8 A = 40 W !!!; That is not environment friendly. Maybe via
  - Relay flips
  - ePaper and embedded-graphics
