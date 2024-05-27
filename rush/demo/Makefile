AS=mips-none-elf-as -mips32
CC=mips-none-elf-gcc
LD=mips-none-elf-ld
OBJCOPY=mips-none-elf-objcopy
CFLAGS=-march=r2000 -static

OBJS=test_emu.o

test_emu.bin: test_emu.elf
	$(OBJCOPY) -O binary test_emu.elf test_emu.bin

test_emu.elf:
	$(CC) $(CFLAGS) test_emu.c -o test_emu.elf -lc -lcfe -lc 

clean:
	rm -f *.o *.elf *.bin
