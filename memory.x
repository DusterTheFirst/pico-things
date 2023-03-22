MEMORY {
    BOOT2(rx)   : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH(rwx)  : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM(rwx)    : ORIGIN = 0x20000000, LENGTH = 256K
    SMALL0(rwx) : ORIGIN = 0x20040000, LENGTH = 4k
    SMALL1(rwx) : ORIGIN = 0x20041000, LENGTH = 4k
}

EXTERN(BOOT2_FIRMWARE)

SECTIONS {
    /* ### Boot loader */
    .boot2 ORIGIN(BOOT2) : {
        KEEP(*(.boot2));
    } > BOOT2

    /* ### Main ram section */
    .ram : {
        *(.ram.*)
        . = ALIGN(4);
    } > RAM

    /* ### Small 4kb memory sections for high bandwidth code or data per core */
    .small.0 : {
        *(.small.0.*)
        . = ALIGN(4);
    } > SMALL0

    .small.1 : {
        *(.small.1.*)
        . = ALIGN(4);
    } > SMALL1
} INSERT BEFORE .text;