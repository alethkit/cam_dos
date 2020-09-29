TARGET = aarch64-unknown-none
LINKER_SCRIPT = src/linker.ld
RELEASE_ELF = target/$(TARGET)/release/cam_dos
DEBUG_ELF = target/$(TARGET)/debug/cam_dos
RELEASE_BIN = release.bin
DEBUG_BIN = debug.bin
RUSTFLAGS = -C link-arg=$(LINKER_SCRIPT)

.PHONY: all debug clean clippy run readelf

all: $(RELEASE_BIN)

debug: $(DEBUG_BIN)

clean:
	cargo clean
	rm -f $(RELEASE_BIN)
	rm -f $(DEBUG_BIN)

clippy:
	RUSTFLAGS="$(RUSTFLAGS)" cargo clippy

run: $(RELEASE_BIN)
	qemu-system-aarch64 -M raspi3 -kernel $(RELEASE_BIN) -nographic

readelf: $(RELEASE_ELF)
	readelf -a $(RELEASE_ELF)


$(RELEASE_BIN): $(RELEASE_ELF)
	rust-objcopy -O binary $(RELEASE_ELF) $(RELEASE_BIN)

$(DEBUG_BIN): $(DEBUG_ELF)
	rust-objcopy -O binary $(DEBUG_ELF) $(DEBUG_BIN)

$(RELEASE_ELF):
	RUSTFLAGS="$(RUSTFLAGS)" cargo build --target $(TARGET) --release

$(DEBUG_ELF):
	RUSTFLAGS="$(RUSTFLAGS)" cargo build --target $(TARGET) 
