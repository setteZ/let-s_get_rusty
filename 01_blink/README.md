# Blink

To setup the project

```bash
$ esp-generate
```

with configuration fonly for ESP32-C6, that's all.

## Dependencies

Let's start adding led abstraction layer and the driver  implementaiton for the on-board RGB led:

```bash
$ cargo add smart_leds
$ cargo add ws2812-spi
```

## Build and flash

First, to acces the /dev/ttyACM0 (this is the serial device I see on my PC when the board is
connected) whiutout root permission, you shall add your user to the ttyACM0 goup:

```bash
stat /dev/ttyACM0 | grep Gid
```

for me the result is `Access: (0660/crw-rw----)  Uid: (    0/    root)   Gid: (  985/    uucp)`,
thus 

```bash
sudo usermod -a -G uucp $USER
```

and reboot. And then, from the blink folder:

```bash
cargo build --release && espflash flash target/riscv32imac-unknown-none-elf/release/blink
```
