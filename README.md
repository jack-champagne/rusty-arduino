# rusty-arduino

Most of this code was found elsewhere [online](https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0) and I encourage people to check them out. (Some info ripped from [here](https://os.phil-opp.com/freestanding-rust-binary/#disabling-unwinding) as well)

# dependencies
- rust toolchain (nightly)
- avr-gcc
- arduino-avr-core (avrdude)

# toolchain setup
```bash
rustup override set nightly
# install other packages above with your package manager
```

# explaination
This project uses the avr HAL (hardware abstraction layer) that @Rahix wrote to safely interface with the arduino peripherials.
The main and panic handler are required to compilation as the embedded project requires them. The entry point does not necessarily need to be
named main as the attribute #[arduino_hal::entry] points the linker script where the projects entry point is.

# flashing
use ./flash.sh to compile and flash code to board
