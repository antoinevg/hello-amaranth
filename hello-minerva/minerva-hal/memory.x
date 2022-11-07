# From: ../build/radiona_ulx3s/software/include/generated/regions.ld
# Can also get it from the litex bios with: `mem_list`
MEMORY {
    rom      : ORIGIN = 0x00000000, LENGTH = 0x00020000
    sram     : ORIGIN = 0x01000000, LENGTH = 0x00002000
    main_ram : ORIGIN = 0x40000000, LENGTH = 0x02000000
    csr      : ORIGIN = 0x82000000, LENGTH = 0x00010000
}

REGION_ALIAS("REGION_TEXT", main_ram);
REGION_ALIAS("REGION_RODATA", main_ram);
REGION_ALIAS("REGION_DATA", main_ram);
REGION_ALIAS("REGION_BSS", main_ram);
REGION_ALIAS("REGION_HEAP", main_ram);
REGION_ALIAS("REGION_STACK", main_ram);

PROVIDE(UART = DefaultHandler);
PROVIDE(TIMER0 = DefaultHandler);
