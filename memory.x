MEMORY {
    BOOT2(rx)       : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH(rwx)      : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM(rwx)        : ORIGIN = 0x20000000, LENGTH = 256K
    SCRATCH_X(rwx)  : ORIGIN = 0x20040000, LENGTH = 4k
    SCRATCH_Y(rwx)  : ORIGIN = 0x20041000, LENGTH = 4k
}

EXTERN(BOOT2_FIRMWARE)

SECTIONS {
    /* ### Boot loader */
    .boot2 ORIGIN(BOOT2) : {
        KEEP(*(.boot2));
    } > BOOT2

    /* ### Small 4kb memory sections for high bandwidth code or data per core */
    .scratch_x : {
        *(.scratch_x.*)
        . = ALIGN(4);
    } > SCRATCH_X

    .scratch_y : {
        *(.scratch_y.*)
        . = ALIGN(4);
    } > SCRATCH_Y
} INSERT BEFORE .text;