
SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* 1. initial Stack Pointer value */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* 2. reset vector */
    KEEP(*(.vector_table.reset_vector));

    /* 3. exception handlers */
    KEEP(*(.vector_table.exceptions));

    /* 4. interrupt handlers */
    KEEP(*(.vector_table.interrupts));
  } > FLASH

  .text : ALIGN(4)
  {
    *(.text .text.*);
  } > FLASH

  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .bss :
  {
    _SBSS = .;
    *(.bss .bss.*);
    _EBSS = .;
  } > RAM

  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    _SDATA = .;
    *(.data .data.*);
    _EDATA = .;
  } > RAM

  _SIDATA = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}
