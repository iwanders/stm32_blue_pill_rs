# Rust on 'Blue Pill'

Using info from [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart/tree/cc19bdda8b93afd458d9c005096571e90b6d2929) and [stm32f1xx-hal](https://github.com/stm32-rs/stm32f1xx-hal/tree/f9b24f4d9bac7fc3c93764bd295125800944f53b).

## Chip used

- Chip: [STM32f103C8T6](https://www.st.com/resource/en/datasheet/stm32f103c8.pdf)
- Instruction set: arm v7m [source](https://en.wikipedia.org/w/index.php?title=ARM_architecture_family&oldid=1097115162#Cores)
- 64k of flash, 20 kb sram.
- Memory start at `0x08000000` according to [flash memory](https://www.st.com/resource/en/programming_manual/pm0075-stm32f10xxx-flash-memory-microcontrollers-stmicroelectronics.pdf).

## Setup Steps

- Install the target: `rustup target add thumbv7m-none-eabi`
- Install gdb for arm: `apt-get install gdb-multiarch`
- `apt-get install openocd`, to use the ST-LINKv2.

## Usage

- Start `openocd` in a terminal from this directory, keep this running in the background.
- Run `cargo run`

