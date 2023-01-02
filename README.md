# stm32l152c-blinky
Just a blinky written in Rust for [STM32l152C Discovery](https://st.com/stm32l152c-discovery)

# Requirements
The following need to be installed
- [GNU Arm Embedded Toolchain](https://developer.arm.com/downloads/-/gnu-rm)
- [Rust](https://www.rust-lang.org/)
- [OpenOCD](https://github.com/xpack-dev-tools/openocd-xpack/releases)
- [STLink driver](https://www.st.com/en/development-tools/stsw-link009.html)

# Installation

Add Cortex-M3 target
```shell
rustup target add thumbv7m-none-eabi
```

# Build
```shell
cargo build
```

# Start OpenOCD
With your stm32l152c-discovery board connected to the PC

```shell
openocd -s $OPENOCD_PATH/share/scripts -f interface/stlink.cfg -f target/stm32l1.cfg
```

Or with dev dependencies installed run
```shell
cargo run-script openocd
```
Note, the command above assumes openocd is in your PATH and installed under `C:\OpenOCD`. Check Cargo.toml to update openocd script.


# Connecting to the debugger
```shell
cargo run
```
## Then connect to the target
On gdb terminal run

```shell
(gdb) target extended-remote :3333
```
Note, the command above assumes default port 3333 is used.

## Load the program
```shell
(gdb) load
```

# Reference

Checkout these links

- [Rust embedded discovery](https://docs.rust-embedded.org/discovery/f3discovery/index.html)
- [stm32l1xx-hal](https://github.com/stm32-rs/stm32l1xx-hal)