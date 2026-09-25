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
