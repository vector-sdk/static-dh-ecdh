RISCV_TARGET := riscv64gc-unknown-none-elf

all:
	cargo build --target $(RISCV_TARGET) --release

clean:
	cargo clean
	rm -f *~ ./src/*~
