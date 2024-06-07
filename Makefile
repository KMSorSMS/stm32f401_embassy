TARGET := $(shell sed -n 's/^name = "\(.*\)"/\1/p' Cargo.toml)
PLATFORM := $(shell sed -n 's/^target = "\(.*\)"/\1/p' .cargo/config.toml)
MODE ?= debug

OBJDUMP := rust-objdump --arch-name=arm
OBJCOPY := rust-objcopy --binary-architecture=arm

ENTRY_PA := 0x08000000

# depend on MODE to determine the mode arg
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

build:
	cargo build $(MODE_ARG) 

bin: $(TARGET)
	@echo "Build done"

$(TARGET): build target/$(PLATFORM)/$(MODE)/$(TARGET)
	$(OBJCOPY) -O binary $(word 2,$^) target/$(PLATFORM)/$(MODE)/$@.bin
debug: build bin
	openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase target/$(PLATFORM)/$(MODE)/${TARGET}.bin 0x8000000"
download: build bin
	openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase target/$(PLATFORM)/$(MODE)/${TARGET}.bin 0x8000000" -c "reset" -c "shutdown"
clean:
	cargo clean
run:
	cargo run $(MODE_ARG)