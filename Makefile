CC=/usr/bin/clang
OBJCOPY=/usr/bin/llvm-objcopy-6.0
LD=ld.lld-6.0

all: kernel8.img

%.o: src/%.S
	$(CC) --target=aarch64 -c $^ -o boot.o

rustpi.o: src/$(wildcard *.rs)
	xargo rustc --release -- --emit obj=rustpi.o --target=aarch64-unknown-none

kernel8.elf: boot.o rustpi.o
	$(LD) -o kernel8.elf -r boot.o rustpi.o -T linker.ld

kernel8.img: kernel8.elf
	$(OBJCOPY) kernel8.elf -O binary kernel8.img 

.PHONY:
clean:
	cargo clean
	rm *.o kernel8.elf kernel8.img
