# Let's get rusty

## Scope

The intent of this repo is to keep track of my approach to the [Embedded](https://rust-embedded.org/)
side of [Rust](https://rust-lang.org). Why Embedded? Because I spent the last 18 years in the
Embedded world with C and now it is time to get my hand... wet with something new (also pushed since
years by a friend).

## The structure

The idea is to keep this README as a roadmap of the trip, while collecting into different subfolders
the projects I am going to develop during this trip. But first of all: which board should I use?

## The board

I just received a chat from the pushing friend: [Microcontrollers with good support for Rust](https://kerkour.com/rust-microcontrollers).
From the list I decided to go with the ESP32-C6 because there a nice little board for around 10
bucks: the [ESP32-C6-Zero](https://docs.waveshare.com/ESP32-C6-Zero?variant=ESP32-C6-Zero). I can't
resist stamp-sized boards!

## Toolchain

At the moment of writing

```bash
$ rustc -V
rustc 1.98.1 (48a229cea 2026-09-01)
```

Let's start from [esp-rust](https://github.com/esp-rs).

```bash
rustup toolchain install stable --component rust-src
rustup target add riscv32imac-unknown-none-elf # For ESP32-C6 and ESP32-H2
cargo install esp-generate --locked
cargo install espflash --locked

```
and the minimal is ready.

# Projects

## Blink

Let's start with the king of al examples: "Hello, world" — in the flavour of the embedded worls, the
[blink](./01_blink).

