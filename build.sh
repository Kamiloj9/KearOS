#!/usr/bin/env bash
set -euo pipefail

cargo build --release
# convert to UF2
mkdir -p output
cp target/thumbv8m.main-none-eabi/release/kear_os output/kear_os.elf
picotool load output/kear_os.elf