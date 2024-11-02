# Pi Pico W Hello World with Rust

Project will toggle an LED on GPIO15 on and off at 500ms intervals and print the number of ticks to the USB Serial Logger.

## Run on Device

- Hold BOOTSEL while connecting device over USB
- Mount Device as USB Storage
- `cargo run --release`

## Connect to USB Serial

```
screen /dev/ttyACM0 115200
```
