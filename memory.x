MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}

EXTERN(BOOT2_FIRMWARE)

SECTIONS {
    /* ### Boot loader */
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2

    /* ### Any high-bandwidth statics or code to put in ram */
    .ram ORIGIN(RAM) :
    {
        KEEP(*(.ram));
    } > RAM

    /*
        TODO: scratch_x and scratch_y (pg 122)
        https://github.com/raspberrypi/pico-sdk/blob/f396d05f8252d4670d4ea05c8b7ac938ef0cd381/src/rp2_common/pico_standard_link/memmap_no_flash.ld#L27-L28
        https://github.com/raspberrypi/pico-sdk/blob/f396d05f8252d4670d4ea05c8b7ac938ef0cd381/src/rp2_common/pico_standard_link/memmap_no_flash.ld#L151-L166
    */
} INSERT BEFORE .text;