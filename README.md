# [`embedded-hal`](https://github.com/japaric/embedded-hal) implementation for ESP8266

This crate implements `embedded-hal` traits for the ESP8266 series of
microcontrollers by using the [Arduino
SDK](https://github.com/esp8266/Arduino). Since LLVM, and therefore rustc, do
not support the Xtensa architecture, they must be transpiled to C using
[mrustc](https://github.com/thepowersgang/mrustc), then compiled using GCC. The
[esp-rs](https://github.com/emosenkis/esp-rs) build script implements this
peculiar build process.

## Supported traits

- `digital::InputPin`

- `digital::OutputPin`

- `digital::StatefulOutputPin`
