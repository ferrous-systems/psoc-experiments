MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x10000000, LENGTH = 32K
  RAM : ORIGIN = 0x08000000, LENGTH = 4K
  APP_FLASH : ORIGIN = 0x10008000, LENGTH = 2048K - 32K
}

SECTIONS {
   .app_flash ORIGIN(APP_FLASH) :
   {
      KEEP(*(.app_flash));
      . = ALIGN(4);
   } > APP_FLASH
} INSERT BEFORE .text;
