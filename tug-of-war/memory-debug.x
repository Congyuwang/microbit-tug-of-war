MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}

/* The entry point is the reset handler */
ENTRY(Reset);

SECTIONS
{
  .text :
  {
    *(.text)
  } > FLASH

  .notes :
  {
    *(.notes);
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.attributes .debug* .comment);
  }
}
