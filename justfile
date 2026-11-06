name := "blink"
build := "debug" # debug | release

elf_target := "target" / "thumbv7m-none-eabi" / build / name
bin_target := "target" / name + ".bin"

cargo_build_flags := if build == "release" { "--release" } else { "" }

build: fmt
	cargo build {{cargo_build_flags}}

# Requires openocd running
debug: build
	arm-none-eabi-gdb -x openocd.gdb -q {{elf_target}}

bin: build
	arm-none-eabi-objcopy -O binary {{elf_target}} {{bin_target}}

disassemble: build
	arm-none-eabi-objdump --disassemble {{elf_target}} | less -S

doc:
	cargo doc --open

fmt:
	find src -type f -name '*.rs' | xargs rustfmt

flash: bin erase
	st-info --descr
	st-flash write {{bin_target}} 0x8000000

erase:
	st-flash erase

# Empties target/ but keeps it a btrfs subvolume, so fs snapshots skip build output.
# `cargo clean` would delete the subvolume, and the next build would recreate a plain directory.
clean:
	#!/usr/bin/env bash
	set -euo pipefail
	# A btrfs subvolume root always has inode 256.
	if [[ -e target && "$(stat -c %i target)" == 256 ]]; then
		find target -mindepth 1 -delete
	else
		rm -rf target
		btrfs subvolume create target
	fi

picocom:
	picocom -b 115200 --imap lfcrlf /dev/ttyACM0
