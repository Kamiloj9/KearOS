/* memory.x for RP2350 + boot2 first */
MEMORY
{
  /* 256-byte second-stage bootloader lives here */
  BOOT2 (rx) : ORIGIN = 0x10000000, LENGTH = 0x100

  /* real flash starts AFTER boot2 */
  FLASH (rx) : ORIGIN = 0x10000100, LENGTH = 4096K - 0x100

  RAM   (rwx): ORIGIN = 0x20000000, LENGTH = 512K
}

SECTIONS
{
  .boot2 ORIGIN(BOOT2) :
  {
    KEEP(*(.boot2));
  } > BOOT2
}